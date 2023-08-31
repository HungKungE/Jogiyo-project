import { PDFDocument, PDFImage } from 'pdf-lib';
import React from 'react';
import { AnyAction, Dispatch } from '@reduxjs/toolkit';
import {
  sendMultiGetNotePageRequest,
  sendMultiGetNoteRequest,
} from '../../api/user_file/note';
import { RenderableNote, RenderableNotePage } from '../../proto/note';
import { RenderableUserFile } from '../../proto/user_file';
import { MultiGetNotePageRequest_RequestItem } from '../../proto/user_file_api';
import { setAlarm } from '../../redux/slices/alarmSlice';
import { offLoading, setLoading } from '../../redux/slices/loadingSlice';
import { AlarmType } from '../../redux/states/alarmState';
import {
  createEmptyNotePage,
  noteCoverToPdfPath,
  noteTemplateToFilePath,
} from '../Note/util/noteUtils';
import { renderBetterNoteArticle } from '../UserGroup/UserGroup/SingleFeed/renderBetterNoteArticle';

export const getNoteInfo = (
  fileInfo: RenderableUserFile,
  dispatch: Dispatch<AnyAction>,
  navigate: (path: string) => void,
  noteInfo: React.MutableRefObject<RenderableNote>,
  notePageList: React.MutableRefObject<RenderableNotePage[]>,
  setCanCreatePdf: () => void
) => {
  dispatch(setLoading('다운로드 중'));

  if (!fileInfo) {
    dispatch(offLoading());
    return;
  }

  sendMultiGetNoteRequest(
    {
      sharedFileUrls: [fileInfo.sharedFileUrl],
    },
    () => {
      dispatch(offLoading());
    }
  )
    .then((response) => {
      if (response.error || !response.notes.length) {
        console.log(response.error);
        dispatch(setAlarm(3000, AlarmType.ERROR, '존재하지 않는 파일입니다.'));
        dispatch(offLoading());
        return;
      }

      noteInfo.current = response.notes[0];
    })
    .then(() => {
      const getNotePageRequestItems = () => {
        const requestItems: MultiGetNotePageRequest_RequestItem[] = [];
        if (noteInfo.current) {
          for (
            let pageIndex = 0;
            pageIndex < noteInfo.current.numTotalPages;
            pageIndex++
          ) {
            requestItems.push({
              sharedFileUrl: fileInfo.sharedFileUrl,
              pageIndex,
            });
          }
        }

        return requestItems;
      };

      sendMultiGetNotePageRequest(
        {
          requestItems: getNotePageRequestItems(),
        },
        () => {
          dispatch(offLoading());
          dispatch(
            setAlarm(
              3000,
              AlarmType.ERROR,
              '다운로드를 하기 위해선 로그인이 필요합니다.'
            )
          );
          navigate('/login');
        }
      ).then((response) => {
        if (response.error) {
          console.log(response.error);
          dispatch(
            setAlarm(
              3000,
              AlarmType.ERROR,
              '다운로드 도중 문제가 발생하였습니다. 잠시 후에 다시 시도해주세요.'
            )
          );
          return;
        }

        notePageList.current = response.responseItems.map(
          (responseItem, pageIndex) => {
            if (responseItem.notePage) {
              return responseItem.notePage;
            } else {
              return createEmptyNotePage(pageIndex, fileInfo.sharedFileUrl);
            }
          }
        );

        notePageList.current = notePageList.current.sort(
          (a, b) => a.pageIndex - b.pageIndex
        );

        setCanCreatePdf();
      });
    });
};

export const loadPdf = async (
  noteInfo: React.MutableRefObject<RenderableNote>,
  notePageList: React.MutableRefObject<RenderableNotePage[]>,
  setPdfDocument: (doc: PDFDocument) => void
) => {
  if (!noteInfo.current) {
    return;
  }

  if (!noteInfo.current.filePath) {
    // note일 때
    // note background -> canvas에 그림 -> pdf
    let createdPdfDocument = await PDFDocument.create();

    for await (const notePage of notePageList.current) {
      const noteBackgroundPath = (() => {
        if (notePage.pageIndex == 0 && noteInfo.current.noteCover) {
          return noteCoverToPdfPath(noteInfo.current.noteCover);
        }

        if (noteInfo.current.noteTemplate) {
          return noteTemplateToFilePath(
            noteInfo.current.noteTemplate,
            noteInfo.current.noteBackgroundColor
          );
        }
      })();

      if (!noteBackgroundPath) {
        return;
      }

      const noteBackgroundBytes = await fetch(noteBackgroundPath).then((res) =>
        res.arrayBuffer()
      );

      const noteBackgroundImage = await createdPdfDocument.embedPdf(
        noteBackgroundBytes
      );

      const page = createdPdfDocument.addPage();

      page.drawPage(noteBackgroundImage[0], {
        width: page.getWidth(),
        height: page.getHeight(),
      });
    }

    const createdPdfBytes = await createdPdfDocument.save();
    const createdPdfBlob = new Blob([createdPdfBytes], {
      type: 'application/pdf',
    });
    const createdPdfUrl = URL.createObjectURL(createdPdfBlob);

    const createdPdfArrayBuffer = await fetch(createdPdfUrl).then((res) =>
      res.arrayBuffer()
    );

    createdPdfDocument = await PDFDocument.load(createdPdfArrayBuffer);

    setPdfDocument(createdPdfDocument);
  } else {
    // pdf일 때
    const loadedPdfArrayBuffer = await fetch(noteInfo.current.filePath).then(
      (res) => res.arrayBuffer()
    );

    const loadedPdfDocument = await PDFDocument.load(loadedPdfArrayBuffer);

    setPdfDocument(loadedPdfDocument);
  }
};

export const loadNote = async (
  pdfDocument: PDFDocument,
  notePageList: React.MutableRefObject<RenderableNotePage[]>,
  imageUrlList: React.MutableRefObject<string[]>,
  imageList: React.MutableRefObject<PDFImage[]>,
  setCanDraw: () => void
) => {
  if (!pdfDocument) {
    return;
  }

  const pdfPages = pdfDocument.getPages();

  const fetchNoteUrl = async () => {
    await Promise.all(
      notePageList.current.map((page) => {
        const noteCanvas = document.createElement('canvas');
        noteCanvas.id = 'temp-thumbnail-image-canvas';
        noteCanvas.style.display = 'none';

        document.body.append(noteCanvas);

        const DPI = window.devicePixelRatio || 2;

        renderBetterNoteArticle(
          page.serializedNote,
          pdfPages[0].getWidth() / DPI,
          pdfPages[0].getHeight() / DPI,
          noteCanvas,
          noteCanvas.id
        );

        noteCanvas.remove();
        imageUrlList.current.push(noteCanvas.toDataURL('image/png'));
      })
    );
    fetchImage();
  };

  const fetchImage = async () => {
    await Promise.all(
      imageUrlList.current.map(async (url, index) => {
        const pngImageBytes = await fetch(url).then((res) => res.arrayBuffer());
        const pngImage = await pdfDocument.embedPng(pngImageBytes);
        imageList.current.push(pngImage);
      })
    );
    setCanDraw();
  };

  await fetchNoteUrl();
};

export const drawNoteOnPdf = async (
  pdfDocument: PDFDocument,
  imageList: React.MutableRefObject<PDFImage[]>,
  setPdfUrl: (url: string) => void
) => {
  if (!pdfDocument) {
    return;
  }

  const pdfPages = pdfDocument.getPages();

  // 각 페이지에 image 그리는 부분
  pdfPages.map((page, index) => {
    page.drawImage(imageList.current[index], {
      width: pdfPages[0].getWidth(),
      height: pdfPages[0].getHeight(),
    });
  });

  const pdfBytes = await pdfDocument.save();
  const pdfBlob = new Blob([pdfBytes], { type: 'application/pdf' });
  const pdfUrl = URL.createObjectURL(pdfBlob);
  setPdfUrl(pdfUrl);
};

export const downloadPdf = (
  noteInfo: React.MutableRefObject<RenderableNote>,
  pdfUrl: string,
  dispatch: Dispatch<AnyAction>
) => {
  if (!noteInfo.current) {
    return;
  }

  const a = document.createElement('a');
  a.href = pdfUrl;
  a.download = noteInfo.current.title;
  document.body.appendChild(a);
  a.click();
  URL.revokeObjectURL(a.href);
  document.body.removeChild(a);
  dispatch(offLoading());
};

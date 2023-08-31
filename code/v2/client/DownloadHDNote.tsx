import init from 'better-note';
import { PDFDocument, PDFImage } from 'pdf-lib';
import React, { useEffect, useRef, useState } from 'react';
import { useDispatch } from 'react-redux';
import { useNavigate } from 'react-router-dom';
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

interface DownloadHDNoteProps {
  fileInfo: RenderableUserFile;
  // undefined 라면 모든 페이지를 가지고 온다.
  pageIndexListToDownload?: number[];
  closeDownloadNoteModal: () => void;
}

const DownloadHDNote: React.FunctionComponent<DownloadHDNoteProps> = ({
  fileInfo,
  pageIndexListToDownload,
  closeDownloadNoteModal,
}) => {
  const [pdfUrl, setPdfUrl] = useState<string>('');
  const [pdfDocument, setPdfDocument] = useState<PDFDocument>();

  const noteInfo = useRef<RenderableNote>();
  const notePageListRef = useRef<RenderableNotePage[]>([]);
  const imageUrlListRef = useRef<string[]>([]);
  const imageListRef = useRef<PDFImage[]>([]);

  const dispatch = useDispatch();
  const navigate = useNavigate();

  const [betterNoteWasmInit, setBetterNoteWasmInit] = useState(false);
  const [createPdf, setCreatePdf] = useState(false);
  const [canDraw, setCandraw] = useState(false);

  const getNoteInfo = () => {
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
          dispatch(
            setAlarm(3000, AlarmType.ERROR, '존재하지 않는 파일입니다.')
          );
          dispatch(offLoading());
          closeDownloadNoteModal();
          return;
        }

        noteInfo.current = response.notes[0];
      })
      .then(() => {
        const getNotePageRequestItems = () => {
          if (pageIndexListToDownload) {
            return pageIndexListToDownload.map((pageIndex) => {
              return {
                pageIndex,
                sharedFileUrl: fileInfo.sharedFileUrl,
              } as MultiGetNotePageRequest_RequestItem;
            });
          } else {
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
          }
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

          notePageListRef.current = response.responseItems.map(
            (responseItem, pageIndex) => {
              if (responseItem.notePage) {
                return responseItem.notePage;
              } else {
                return createEmptyNotePage(pageIndex, fileInfo.sharedFileUrl);
              }
            }
          );

          notePageListRef.current = notePageListRef.current.sort(
            (a, b) => a.pageIndex - b.pageIndex
          );

          setCreatePdf(true);
        });
      });
  };

  const loadPdf = async () => {
    if (!noteInfo.current) {
      return;
    }

    if (!noteInfo.current.filePath) {
      // note일 때
      // note background -> canvas에 그림 -> pdf
      let createdPdfDocument = await PDFDocument.create();

      for await (const notePage of notePageListRef.current) {
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

        const noteBackgroundBytes = await fetch(noteBackgroundPath).then(
          (res) => res.arrayBuffer()
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

  const loadNote = async () => {
    if (!pdfDocument) {
      return;
    }

    const pdfPages = pdfDocument.getPages();

    const fetchNoteUrl = async () => {
      await Promise.all(
        notePageListRef.current.map((page) => {
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
          imageUrlListRef.current.push(noteCanvas.toDataURL('image/png'));
        })
      );
      fetchImage();
    };

    const fetchImage = async () => {
      await Promise.all(
        imageUrlListRef.current.map(async (url, index) => {
          const pngImageBytes = await fetch(url).then((res) =>
            res.arrayBuffer()
          );
          const pngImage = await pdfDocument.embedPng(pngImageBytes);
          imageListRef.current.push(pngImage);
        })
      );
      setCandraw(true);
    };

    await fetchNoteUrl();
  };

  const drawNoteOnPdf = async () => {
    if (!pdfDocument) {
      return;
    }

    const pdfPages = pdfDocument.getPages();

    // 각 페이지에 image 그리는 부분
    pdfPages.map((page, index) => {
      page.drawImage(imageListRef.current[index], {
        width: pdfPages[0].getWidth(),
        height: pdfPages[0].getHeight(),
      });
    });

    const pdfBytes = await pdfDocument.save();
    const pdfBlob = new Blob([pdfBytes], { type: 'application/pdf' });
    const pdfUrl = URL.createObjectURL(pdfBlob);
    setPdfUrl(pdfUrl);
  };

  const downloadPdf = () => {
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
    closeDownloadNoteModal();
  };

  useEffect(() => {
    // betterNote 초기화
    init().then(() => setBetterNoteWasmInit(true));
    // noteInfo, notePageListInfo 를 가져옴
    getNoteInfo();
    // createPdf = true 됨
  }, []);

  useEffect(() => {
    if (!noteInfo.current || !createPdf || !betterNoteWasmInit) {
      return;
    }

    if (!notePageListRef.current.length) {
      dispatch(
        setAlarm(3000, AlarmType.ERROR, '페이지가 존재하지 않는 노트입니다.')
      );
      return;
    }

    loadPdf();
  }, [createPdf, betterNoteWasmInit]);

  useEffect(() => {
    if (!pdfDocument) {
      return;
    }
    loadNote();
  }, [pdfDocument]);

  useEffect(() => {
    if (!canDraw) {
      return;
    }
    drawNoteOnPdf();
  }, [canDraw]);

  useEffect(() => {
    if (!pdfUrl) {
      return;
    }
    downloadPdf();
  }, [pdfUrl]);

  return <React.Fragment></React.Fragment>;
};

export default DownloadHDNote;

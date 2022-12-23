use byteorder::ByteOrder;

use futures::AsyncReadExt;
use futures::StreamExt;
use serde::Serialize;

use super::header::BitpixValue;

use std::fmt::Debug;
use futures::AsyncBufRead;

/// Abstraction for reading a data block
pub trait AsyncDataRead: AsyncBufRead {
    type Data: Debug;

    unsafe fn read_data_block(self, bitpix: BitpixValue, num_pixels: usize) -> Self::Data where Self: Sized;
}

impl<R> AsyncDataRead for futures::io::BufReader<R>
where
    R: futures::AsyncRead + std::marker::Unpin + Debug
{
    type Data = DataOwned<Self>;

    unsafe fn read_data_block(self, bitpix: BitpixValue, num_pixels: usize) -> Self::Data {
        match bitpix {
            BitpixValue::U8 => DataOwned::U8(DataOwnedSt::new(self, num_pixels)),
            BitpixValue::I16 => DataOwned::I16(DataOwnedSt::new(self, num_pixels)),
            BitpixValue::I32 => DataOwned::I32(DataOwnedSt::new(self, num_pixels)),
            BitpixValue::I64 => DataOwned::I64(DataOwnedSt::new(self, num_pixels)),
            BitpixValue::F32 => DataOwned::F32(DataOwnedSt::new(self, num_pixels)),
            BitpixValue::F64 => DataOwned::F64(DataOwnedSt::new(self, num_pixels)),
        }
    }
}

#[derive(Serialize)]
#[derive(Debug)]
pub enum DataOwned<R>
where
    R: AsyncBufRead
{
    U8(DataOwnedSt<R, u8>),
    I16(DataOwnedSt<R, i16>),
    I32(DataOwnedSt<R, i32>),
    I64(DataOwnedSt<R, i64>),
    F32(DataOwnedSt<R, f32>),
    F64(DataOwnedSt<R, f64>),
}

#[derive(Serialize)]
#[derive(Debug)]
pub struct DataOwnedSt<R, T>
where
    R: futures::AsyncBufRead
{
    reader: R,
    num_pixels: usize,
    counter: usize,
    phantom: std::marker::PhantomData<T>,
}

impl<R, T> DataOwnedSt<R, T>
where
    R: futures::AsyncBufRead
{
    fn new(reader: R, num_pixels: usize) -> Self {
        let counter = 0;
        Self {
            reader,
            counter,
            num_pixels,
            phantom: std::marker::PhantomData
        }
    }
}

use std::pin::Pin;
use futures::task::Context;
use futures::task::Poll;
use futures::Future;

impl<R> futures::Stream for DataOwnedSt<R, u8>
where
    R: futures::AsyncBufReadExt + std::marker::Unpin
{
    /// The type of the value yielded by the stream.
    type Item = u8;

    /// Attempt to resolve the next item in the stream.
    /// Returns `Poll::Pending` if not ready, `Poll::Ready(Some(x))` if a value
    /// is ready, and `Poll::Ready(None)` if the stream has completed.
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.num_pixels == self.counter {
            // The stream has finished
            Poll::Ready(None)
        } else {
            let mut buf = [0_u8; 1];

            let mut reader_exact = self.reader.read_exact(&mut buf);
            match Pin::new(&mut reader_exact).poll(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(Err(_)) => Poll::Ready(None),
                Poll::Ready(Ok(())) => {
                    self.counter += 1;
                    Poll::Ready(Some(buf[0]))
                }
            }
        }
    }
}

impl<R> futures::Stream for DataOwnedSt<R, i16>
where
    R: futures::AsyncBufReadExt + std::marker::Unpin
{
    /// The type of the value yielded by the stream.
    type Item = i16;

    /// Attempt to resolve the next item in the stream.
    /// Returns `Poll::Pending` if not ready, `Poll::Ready(Some(x))` if a value
    /// is ready, and `Poll::Ready(None)` if the stream has completed.
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.num_pixels == self.counter {
            // The stream has finished
            Poll::Ready(None)
        } else {
            let mut buf = [0_u8; 2];
            let mut reader_exact = self.reader.read_exact(&mut buf);
            match Pin::new(&mut reader_exact).poll(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(Err(_)) => Poll::Ready(None),
                Poll::Ready(Ok(())) => {
                    let item = byteorder::BigEndian::read_i16(&buf);
                    self.counter += 1;
                    Poll::Ready(Some(item))
                }
            }
        }
    }
}

impl<R> futures::Stream for DataOwnedSt<R, i32>
where
    R: futures::AsyncBufReadExt + std::marker::Unpin
{
    /// The type of the value yielded by the stream.
    type Item = i32;

    /// Attempt to resolve the next item in the stream.
    /// Returns `Poll::Pending` if not ready, `Poll::Ready(Some(x))` if a value
    /// is ready, and `Poll::Ready(None)` if the stream has completed.
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.num_pixels == self.counter {
            // The stream has finished
            Poll::Ready(None)
        } else {
            let mut buf = [0_u8; 4];
            let mut reader_exact = self.reader.read_exact(&mut buf);
            match Pin::new(&mut reader_exact).poll(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(Err(_)) => Poll::Ready(None),
                Poll::Ready(Ok(())) => {
                    let item = byteorder::BigEndian::read_i32(&buf);
                    self.counter += 1;
                    Poll::Ready(Some(item))
                }
            }
        }
    }
}

impl<R> futures::Stream for DataOwnedSt<R, i64>
where
    R: futures::AsyncBufReadExt + std::marker::Unpin
{
    /// The type of the value yielded by the stream.
    type Item = i64;

    /// Attempt to resolve the next item in the stream.
    /// Returns `Poll::Pending` if not ready, `Poll::Ready(Some(x))` if a value
    /// is ready, and `Poll::Ready(None)` if the stream has completed.
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.num_pixels == self.counter {
            // The stream has finished
            Poll::Ready(None)
        } else {
            let mut buf = [0_u8; 8];
            let mut reader_exact = self.reader.read_exact(&mut buf);
            match Pin::new(&mut reader_exact).poll(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(Err(_)) => Poll::Ready(None),
                Poll::Ready(Ok(())) => {
                    let item = byteorder::BigEndian::read_i64(&buf);
                    self.counter += 1;
                    Poll::Ready(Some(item))
                }
            }
        }
    }
}

impl<R> futures::Stream for DataOwnedSt<R, f32>
where
    R: futures::AsyncBufReadExt + std::marker::Unpin
{
    /// The type of the value yielded by the stream.
    type Item = f32;

    /// Attempt to resolve the next item in the stream.
    /// Returns `Poll::Pending` if not ready, `Poll::Ready(Some(x))` if a value
    /// is ready, and `Poll::Ready(None)` if the stream has completed.
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.num_pixels == self.counter {
            // The stream has finished
            Poll::Ready(None)
        } else {
            let mut buf = [0_u8; 4];
            let mut reader_exact = self.reader.read_exact(&mut buf);
            match Pin::new(&mut reader_exact).poll(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(Err(_)) => Poll::Ready(None),
                Poll::Ready(Ok(())) => {
                    let item = byteorder::BigEndian::read_f32(&buf);
                    self.counter += 1;
                    Poll::Ready(Some(item))
                }
            }
        }
    }
}

impl<R> futures::Stream for DataOwnedSt<R, f64>
where
    R: futures::AsyncBufReadExt + std::marker::Unpin
{
    /// The type of the value yielded by the stream.
    type Item = f64;

    /// Attempt to resolve the next item in the stream.
    /// Returns `Poll::Pending` if not ready, `Poll::Ready(Some(x))` if a value
    /// is ready, and `Poll::Ready(None)` if the stream has completed.
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.num_pixels == self.counter {
            // The stream has finished
            Poll::Ready(None)
        } else {
            let mut buf = [0_u8; 8];
            let mut reader_exact = self.reader.read_exact(&mut buf);
            match Pin::new(&mut reader_exact).poll(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(Err(_)) => Poll::Ready(None),
                Poll::Ready(Ok(())) => {
                    let item = byteorder::BigEndian::read_f64(&buf);
                    self.counter += 1;
                    Poll::Ready(Some(item))
                }
            }
        }
    }
}
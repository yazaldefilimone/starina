//! DO NOT EDIT: This file is auto-generated by ftl_idlc.
#![no_std]
#![feature(const_mut_refs)]
#![feature(const_intrinsic_copy)]

use ftl_types::message::MessageBuffer;
use ftl_types::message::MessageDeserialize;
use ftl_types::message::MessageInfo;
use ftl_types::message::MessageSerialize;
use ftl_types::message::MovedHandle;

pub mod protocols {
    use super::*;

    pub mod autopilot {
        use super::*;

        #[repr(C)]
        pub struct NewclientRequest {
            pub handle: MovedHandle,
        }

        #[repr(C)]
        struct InlinedPartNewclientRequest {}

        impl MessageSerialize for NewclientRequest {
            const MSGINFO: MessageInfo = MessageInfo::from_raw(
                (1 << 14) | (1 << 12) | ::core::mem::size_of::<NewclientRequest>() as isize,
            );

            fn serialize(self, buffer: &mut MessageBuffer) {
                // TODO: Make this a compile-time assertion.
                debug_assert!(::core::mem::size_of::<NewclientRequest>() < 1 << 12);

                // The actual serialization is done in this const fn. This is to
                // ensure the serialization can be done with const operations.
                const fn do_serialize(this: NewclientRequest, buffer: &mut MessageBuffer) {
                    let object = InlinedPartNewclientRequest {};

                    let dst = buffer as *mut _ as *mut InlinedPartNewclientRequest;
                    let src = &object as *const _ as *const InlinedPartNewclientRequest;

                    unsafe {
                        core::ptr::copy_nonoverlapping::<InlinedPartNewclientRequest>(src, dst, 1);
                    }

                    // FIXME: Support multiple handles.
                    debug_assert!(
                        MessageInfo::from_raw(NewclientRequest::MSGINFO.as_raw()).num_handles()
                            <= 1
                    );

                    buffer.handles[0] = this.handle.0;

                    // Don't call destructors on handles transferred to this buffer.
                    core::mem::forget(this);
                }

                do_serialize(self, buffer)
            }
        }

        impl MessageDeserialize for NewclientRequest {
            type Reader<'a> = NewclientRequestReader<'a>;

            fn deserialize<'a>(
                buffer: &'a MessageBuffer,
                msginfo: MessageInfo,
            ) -> Option<NewclientRequestReader<'a>> {
                if msginfo == Self::MSGINFO {
                    Some(NewclientRequestReader { buffer })
                } else {
                    None
                }
            }
        }

        pub struct NewclientRequestReader<'a> {
            #[allow(dead_code)]
            buffer: &'a MessageBuffer,
        }

        impl<'a> NewclientRequestReader<'a> {
            #[allow(dead_code)]
            fn as_ref(&self, buffer: &'a MessageBuffer) -> &'a InlinedPartNewclientRequest {
                unsafe { &*(buffer as *const _ as *const InlinedPartNewclientRequest) }
            }

            pub fn handle(&self) -> ftl_types::handle::HandleId {
                // TODO: return OwnedHandle
                // FIXME: Support multiple handles.
                self.buffer.handles[0]
            }
        }

        #[repr(C)]
        pub struct NewclientReply {}

        #[repr(C)]
        struct InlinedPartNewclientReply {}

        impl MessageSerialize for NewclientReply {
            const MSGINFO: MessageInfo = MessageInfo::from_raw(
                (2 << 14) | (0 << 12) | ::core::mem::size_of::<NewclientReply>() as isize,
            );

            fn serialize(self, buffer: &mut MessageBuffer) {
                // TODO: Make this a compile-time assertion.
                debug_assert!(::core::mem::size_of::<NewclientReply>() < 1 << 12);

                // The actual serialization is done in this const fn. This is to
                // ensure the serialization can be done with const operations.
                const fn do_serialize(this: NewclientReply, buffer: &mut MessageBuffer) {
                    let object = InlinedPartNewclientReply {};

                    let dst = buffer as *mut _ as *mut InlinedPartNewclientReply;
                    let src = &object as *const _ as *const InlinedPartNewclientReply;

                    unsafe {
                        core::ptr::copy_nonoverlapping::<InlinedPartNewclientReply>(src, dst, 1);
                    }

                    // FIXME: Support multiple handles.
                    debug_assert!(
                        MessageInfo::from_raw(NewclientReply::MSGINFO.as_raw()).num_handles() <= 1
                    );

                    // Don't call destructors on handles transferred to this buffer.
                    core::mem::forget(this);
                }

                do_serialize(self, buffer)
            }
        }

        impl MessageDeserialize for NewclientReply {
            type Reader<'a> = NewclientReplyReader<'a>;

            fn deserialize<'a>(
                buffer: &'a MessageBuffer,
                msginfo: MessageInfo,
            ) -> Option<NewclientReplyReader<'a>> {
                if msginfo == Self::MSGINFO {
                    Some(NewclientReplyReader { buffer })
                } else {
                    None
                }
            }
        }

        pub struct NewclientReplyReader<'a> {
            #[allow(dead_code)]
            buffer: &'a MessageBuffer,
        }

        impl<'a> NewclientReplyReader<'a> {
            #[allow(dead_code)]
            fn as_ref(&self, buffer: &'a MessageBuffer) -> &'a InlinedPartNewclientReply {
                unsafe { &*(buffer as *const _ as *const InlinedPartNewclientReply) }
            }
        }
    }

    pub mod ping {
        use super::*;

        #[repr(C)]
        pub struct PingRequest {
            pub int_value1: i32,

            pub bytes_value1: ftl_types::idl::BytesField<16>,
        }

        #[repr(C)]
        struct InlinedPartPingRequest {
            pub int_value1: i32,

            pub bytes_value1: ftl_types::idl::BytesField<16>,
        }

        impl MessageSerialize for PingRequest {
            const MSGINFO: MessageInfo = MessageInfo::from_raw(
                (3 << 14) | (0 << 12) | ::core::mem::size_of::<PingRequest>() as isize,
            );

            fn serialize(self, buffer: &mut MessageBuffer) {
                // TODO: Make this a compile-time assertion.
                debug_assert!(::core::mem::size_of::<PingRequest>() < 1 << 12);

                // The actual serialization is done in this const fn. This is to
                // ensure the serialization can be done with const operations.
                const fn do_serialize(this: PingRequest, buffer: &mut MessageBuffer) {
                    let object = InlinedPartPingRequest {
                        int_value1: this.int_value1,

                        bytes_value1: this.bytes_value1,
                    };

                    let dst = buffer as *mut _ as *mut InlinedPartPingRequest;
                    let src = &object as *const _ as *const InlinedPartPingRequest;

                    unsafe {
                        core::ptr::copy_nonoverlapping::<InlinedPartPingRequest>(src, dst, 1);
                    }

                    // FIXME: Support multiple handles.
                    debug_assert!(
                        MessageInfo::from_raw(PingRequest::MSGINFO.as_raw()).num_handles() <= 1
                    );

                    // Don't call destructors on handles transferred to this buffer.
                    core::mem::forget(this);
                }

                do_serialize(self, buffer)
            }
        }

        impl MessageDeserialize for PingRequest {
            type Reader<'a> = PingRequestReader<'a>;

            fn deserialize<'a>(
                buffer: &'a MessageBuffer,
                msginfo: MessageInfo,
            ) -> Option<PingRequestReader<'a>> {
                if msginfo == Self::MSGINFO {
                    Some(PingRequestReader { buffer })
                } else {
                    None
                }
            }
        }

        pub struct PingRequestReader<'a> {
            #[allow(dead_code)]
            buffer: &'a MessageBuffer,
        }

        impl<'a> PingRequestReader<'a> {
            #[allow(dead_code)]
            fn as_ref(&self, buffer: &'a MessageBuffer) -> &'a InlinedPartPingRequest {
                unsafe { &*(buffer as *const _ as *const InlinedPartPingRequest) }
            }

            pub fn int_value1(&self) -> i32 {
                let m = self.as_ref(self.buffer);
                m.int_value1
            }

            pub fn bytes_value1(&self) -> ftl_types::idl::BytesField<16> {
                let m = self.as_ref(self.buffer);
                m.bytes_value1
            }
        }

        #[repr(C)]
        pub struct PingReply {
            pub int_value2: i32,

            pub str_value2: ftl_types::idl::StringField<32>,
        }

        #[repr(C)]
        struct InlinedPartPingReply {
            pub int_value2: i32,

            pub str_value2: ftl_types::idl::StringField<32>,
        }

        impl MessageSerialize for PingReply {
            const MSGINFO: MessageInfo = MessageInfo::from_raw(
                (4 << 14) | (0 << 12) | ::core::mem::size_of::<PingReply>() as isize,
            );

            fn serialize(self, buffer: &mut MessageBuffer) {
                // TODO: Make this a compile-time assertion.
                debug_assert!(::core::mem::size_of::<PingReply>() < 1 << 12);

                // The actual serialization is done in this const fn. This is to
                // ensure the serialization can be done with const operations.
                const fn do_serialize(this: PingReply, buffer: &mut MessageBuffer) {
                    let object = InlinedPartPingReply {
                        int_value2: this.int_value2,

                        str_value2: this.str_value2,
                    };

                    let dst = buffer as *mut _ as *mut InlinedPartPingReply;
                    let src = &object as *const _ as *const InlinedPartPingReply;

                    unsafe {
                        core::ptr::copy_nonoverlapping::<InlinedPartPingReply>(src, dst, 1);
                    }

                    // FIXME: Support multiple handles.
                    debug_assert!(
                        MessageInfo::from_raw(PingReply::MSGINFO.as_raw()).num_handles() <= 1
                    );

                    // Don't call destructors on handles transferred to this buffer.
                    core::mem::forget(this);
                }

                do_serialize(self, buffer)
            }
        }

        impl MessageDeserialize for PingReply {
            type Reader<'a> = PingReplyReader<'a>;

            fn deserialize<'a>(
                buffer: &'a MessageBuffer,
                msginfo: MessageInfo,
            ) -> Option<PingReplyReader<'a>> {
                if msginfo == Self::MSGINFO {
                    Some(PingReplyReader { buffer })
                } else {
                    None
                }
            }
        }

        pub struct PingReplyReader<'a> {
            #[allow(dead_code)]
            buffer: &'a MessageBuffer,
        }

        impl<'a> PingReplyReader<'a> {
            #[allow(dead_code)]
            fn as_ref(&self, buffer: &'a MessageBuffer) -> &'a InlinedPartPingReply {
                unsafe { &*(buffer as *const _ as *const InlinedPartPingReply) }
            }

            pub fn int_value2(&self) -> i32 {
                let m = self.as_ref(self.buffer);
                m.int_value2
            }

            pub fn str_value2(&self) -> ftl_types::idl::StringField<32> {
                let m = self.as_ref(self.buffer);
                m.str_value2
            }
        }
    }

    pub mod ethernet_device {
        use super::*;

        #[repr(C)]
        pub struct Tx {
            pub payload: ftl_types::idl::BytesField<1514>,
        }

        #[repr(C)]
        struct InlinedPartTx {
            pub payload: ftl_types::idl::BytesField<1514>,
        }

        impl MessageSerialize for Tx {
            const MSGINFO: MessageInfo = MessageInfo::from_raw(
                (5 << 14) | (0 << 12) | ::core::mem::size_of::<Tx>() as isize,
            );

            fn serialize(self, buffer: &mut MessageBuffer) {
                // TODO: Make this a compile-time assertion.
                debug_assert!(::core::mem::size_of::<Tx>() < 1 << 12);

                // The actual serialization is done in this const fn. This is to
                // ensure the serialization can be done with const operations.
                const fn do_serialize(this: Tx, buffer: &mut MessageBuffer) {
                    let object = InlinedPartTx {
                        payload: this.payload,
                    };

                    let dst = buffer as *mut _ as *mut InlinedPartTx;
                    let src = &object as *const _ as *const InlinedPartTx;

                    unsafe {
                        core::ptr::copy_nonoverlapping::<InlinedPartTx>(src, dst, 1);
                    }

                    // FIXME: Support multiple handles.
                    debug_assert!(MessageInfo::from_raw(Tx::MSGINFO.as_raw()).num_handles() <= 1);

                    // Don't call destructors on handles transferred to this buffer.
                    core::mem::forget(this);
                }

                do_serialize(self, buffer)
            }
        }

        impl MessageDeserialize for Tx {
            type Reader<'a> = TxReader<'a>;

            fn deserialize<'a>(
                buffer: &'a MessageBuffer,
                msginfo: MessageInfo,
            ) -> Option<TxReader<'a>> {
                if msginfo == Self::MSGINFO {
                    Some(TxReader { buffer })
                } else {
                    None
                }
            }
        }

        pub struct TxReader<'a> {
            #[allow(dead_code)]
            buffer: &'a MessageBuffer,
        }

        impl<'a> TxReader<'a> {
            #[allow(dead_code)]
            fn as_ref(&self, buffer: &'a MessageBuffer) -> &'a InlinedPartTx {
                unsafe { &*(buffer as *const _ as *const InlinedPartTx) }
            }

            pub fn payload(&self) -> ftl_types::idl::BytesField<1514> {
                let m = self.as_ref(self.buffer);
                m.payload
            }
        }

        #[repr(C)]
        pub struct Rx {
            pub payload: ftl_types::idl::BytesField<1514>,
        }

        #[repr(C)]
        struct InlinedPartRx {
            pub payload: ftl_types::idl::BytesField<1514>,
        }

        impl MessageSerialize for Rx {
            const MSGINFO: MessageInfo = MessageInfo::from_raw(
                (6 << 14) | (0 << 12) | ::core::mem::size_of::<Rx>() as isize,
            );

            fn serialize(self, buffer: &mut MessageBuffer) {
                // TODO: Make this a compile-time assertion.
                debug_assert!(::core::mem::size_of::<Rx>() < 1 << 12);

                // The actual serialization is done in this const fn. This is to
                // ensure the serialization can be done with const operations.
                const fn do_serialize(this: Rx, buffer: &mut MessageBuffer) {
                    let object = InlinedPartRx {
                        payload: this.payload,
                    };

                    let dst = buffer as *mut _ as *mut InlinedPartRx;
                    let src = &object as *const _ as *const InlinedPartRx;

                    unsafe {
                        core::ptr::copy_nonoverlapping::<InlinedPartRx>(src, dst, 1);
                    }

                    // FIXME: Support multiple handles.
                    debug_assert!(MessageInfo::from_raw(Rx::MSGINFO.as_raw()).num_handles() <= 1);

                    // Don't call destructors on handles transferred to this buffer.
                    core::mem::forget(this);
                }

                do_serialize(self, buffer)
            }
        }

        impl MessageDeserialize for Rx {
            type Reader<'a> = RxReader<'a>;

            fn deserialize<'a>(
                buffer: &'a MessageBuffer,
                msginfo: MessageInfo,
            ) -> Option<RxReader<'a>> {
                if msginfo == Self::MSGINFO {
                    Some(RxReader { buffer })
                } else {
                    None
                }
            }
        }

        pub struct RxReader<'a> {
            #[allow(dead_code)]
            buffer: &'a MessageBuffer,
        }

        impl<'a> RxReader<'a> {
            #[allow(dead_code)]
            fn as_ref(&self, buffer: &'a MessageBuffer) -> &'a InlinedPartRx {
                unsafe { &*(buffer as *const _ as *const InlinedPartRx) }
            }

            pub fn payload(&self) -> ftl_types::idl::BytesField<1514> {
                let m = self.as_ref(self.buffer);
                m.payload
            }
        }
    }

    pub mod tcpip {
        use super::*;

        #[repr(C)]
        pub struct TcpClosed {}

        #[repr(C)]
        struct InlinedPartTcpClosed {}

        impl MessageSerialize for TcpClosed {
            const MSGINFO: MessageInfo = MessageInfo::from_raw(
                (7 << 14) | (0 << 12) | ::core::mem::size_of::<TcpClosed>() as isize,
            );

            fn serialize(self, buffer: &mut MessageBuffer) {
                // TODO: Make this a compile-time assertion.
                debug_assert!(::core::mem::size_of::<TcpClosed>() < 1 << 12);

                // The actual serialization is done in this const fn. This is to
                // ensure the serialization can be done with const operations.
                const fn do_serialize(this: TcpClosed, buffer: &mut MessageBuffer) {
                    let object = InlinedPartTcpClosed {};

                    let dst = buffer as *mut _ as *mut InlinedPartTcpClosed;
                    let src = &object as *const _ as *const InlinedPartTcpClosed;

                    unsafe {
                        core::ptr::copy_nonoverlapping::<InlinedPartTcpClosed>(src, dst, 1);
                    }

                    // FIXME: Support multiple handles.
                    debug_assert!(
                        MessageInfo::from_raw(TcpClosed::MSGINFO.as_raw()).num_handles() <= 1
                    );

                    // Don't call destructors on handles transferred to this buffer.
                    core::mem::forget(this);
                }

                do_serialize(self, buffer)
            }
        }

        impl MessageDeserialize for TcpClosed {
            type Reader<'a> = TcpClosedReader<'a>;

            fn deserialize<'a>(
                buffer: &'a MessageBuffer,
                msginfo: MessageInfo,
            ) -> Option<TcpClosedReader<'a>> {
                if msginfo == Self::MSGINFO {
                    Some(TcpClosedReader { buffer })
                } else {
                    None
                }
            }
        }

        pub struct TcpClosedReader<'a> {
            #[allow(dead_code)]
            buffer: &'a MessageBuffer,
        }

        impl<'a> TcpClosedReader<'a> {
            #[allow(dead_code)]
            fn as_ref(&self, buffer: &'a MessageBuffer) -> &'a InlinedPartTcpClosed {
                unsafe { &*(buffer as *const _ as *const InlinedPartTcpClosed) }
            }
        }

        #[repr(C)]
        pub struct TcpAccepted {
            pub sock: MovedHandle,
        }

        #[repr(C)]
        struct InlinedPartTcpAccepted {}

        impl MessageSerialize for TcpAccepted {
            const MSGINFO: MessageInfo = MessageInfo::from_raw(
                (8 << 14) | (1 << 12) | ::core::mem::size_of::<TcpAccepted>() as isize,
            );

            fn serialize(self, buffer: &mut MessageBuffer) {
                // TODO: Make this a compile-time assertion.
                debug_assert!(::core::mem::size_of::<TcpAccepted>() < 1 << 12);

                // The actual serialization is done in this const fn. This is to
                // ensure the serialization can be done with const operations.
                const fn do_serialize(this: TcpAccepted, buffer: &mut MessageBuffer) {
                    let object = InlinedPartTcpAccepted {};

                    let dst = buffer as *mut _ as *mut InlinedPartTcpAccepted;
                    let src = &object as *const _ as *const InlinedPartTcpAccepted;

                    unsafe {
                        core::ptr::copy_nonoverlapping::<InlinedPartTcpAccepted>(src, dst, 1);
                    }

                    // FIXME: Support multiple handles.
                    debug_assert!(
                        MessageInfo::from_raw(TcpAccepted::MSGINFO.as_raw()).num_handles() <= 1
                    );

                    buffer.handles[0] = this.sock.0;

                    // Don't call destructors on handles transferred to this buffer.
                    core::mem::forget(this);
                }

                do_serialize(self, buffer)
            }
        }

        impl MessageDeserialize for TcpAccepted {
            type Reader<'a> = TcpAcceptedReader<'a>;

            fn deserialize<'a>(
                buffer: &'a MessageBuffer,
                msginfo: MessageInfo,
            ) -> Option<TcpAcceptedReader<'a>> {
                if msginfo == Self::MSGINFO {
                    Some(TcpAcceptedReader { buffer })
                } else {
                    None
                }
            }
        }

        pub struct TcpAcceptedReader<'a> {
            #[allow(dead_code)]
            buffer: &'a MessageBuffer,
        }

        impl<'a> TcpAcceptedReader<'a> {
            #[allow(dead_code)]
            fn as_ref(&self, buffer: &'a MessageBuffer) -> &'a InlinedPartTcpAccepted {
                unsafe { &*(buffer as *const _ as *const InlinedPartTcpAccepted) }
            }

            pub fn sock(&self) -> ftl_types::handle::HandleId {
                // TODO: return OwnedHandle
                // FIXME: Support multiple handles.
                self.buffer.handles[0]
            }
        }

        #[repr(C)]
        pub struct TcpReceived {
            pub data: ftl_types::idl::BytesField<2048>,
        }

        #[repr(C)]
        struct InlinedPartTcpReceived {
            pub data: ftl_types::idl::BytesField<2048>,
        }

        impl MessageSerialize for TcpReceived {
            const MSGINFO: MessageInfo = MessageInfo::from_raw(
                (9 << 14) | (0 << 12) | ::core::mem::size_of::<TcpReceived>() as isize,
            );

            fn serialize(self, buffer: &mut MessageBuffer) {
                // TODO: Make this a compile-time assertion.
                debug_assert!(::core::mem::size_of::<TcpReceived>() < 1 << 12);

                // The actual serialization is done in this const fn. This is to
                // ensure the serialization can be done with const operations.
                const fn do_serialize(this: TcpReceived, buffer: &mut MessageBuffer) {
                    let object = InlinedPartTcpReceived { data: this.data };

                    let dst = buffer as *mut _ as *mut InlinedPartTcpReceived;
                    let src = &object as *const _ as *const InlinedPartTcpReceived;

                    unsafe {
                        core::ptr::copy_nonoverlapping::<InlinedPartTcpReceived>(src, dst, 1);
                    }

                    // FIXME: Support multiple handles.
                    debug_assert!(
                        MessageInfo::from_raw(TcpReceived::MSGINFO.as_raw()).num_handles() <= 1
                    );

                    // Don't call destructors on handles transferred to this buffer.
                    core::mem::forget(this);
                }

                do_serialize(self, buffer)
            }
        }

        impl MessageDeserialize for TcpReceived {
            type Reader<'a> = TcpReceivedReader<'a>;

            fn deserialize<'a>(
                buffer: &'a MessageBuffer,
                msginfo: MessageInfo,
            ) -> Option<TcpReceivedReader<'a>> {
                if msginfo == Self::MSGINFO {
                    Some(TcpReceivedReader { buffer })
                } else {
                    None
                }
            }
        }

        pub struct TcpReceivedReader<'a> {
            #[allow(dead_code)]
            buffer: &'a MessageBuffer,
        }

        impl<'a> TcpReceivedReader<'a> {
            #[allow(dead_code)]
            fn as_ref(&self, buffer: &'a MessageBuffer) -> &'a InlinedPartTcpReceived {
                unsafe { &*(buffer as *const _ as *const InlinedPartTcpReceived) }
            }

            pub fn data(&self) -> ftl_types::idl::BytesField<2048> {
                let m = self.as_ref(self.buffer);
                m.data
            }
        }

        #[repr(C)]
        pub struct TcpListenRequest {
            pub port: u16,
        }

        #[repr(C)]
        struct InlinedPartTcpListenRequest {
            pub port: u16,
        }

        impl MessageSerialize for TcpListenRequest {
            const MSGINFO: MessageInfo = MessageInfo::from_raw(
                (10 << 14) | (0 << 12) | ::core::mem::size_of::<TcpListenRequest>() as isize,
            );

            fn serialize(self, buffer: &mut MessageBuffer) {
                // TODO: Make this a compile-time assertion.
                debug_assert!(::core::mem::size_of::<TcpListenRequest>() < 1 << 12);

                // The actual serialization is done in this const fn. This is to
                // ensure the serialization can be done with const operations.
                const fn do_serialize(this: TcpListenRequest, buffer: &mut MessageBuffer) {
                    let object = InlinedPartTcpListenRequest { port: this.port };

                    let dst = buffer as *mut _ as *mut InlinedPartTcpListenRequest;
                    let src = &object as *const _ as *const InlinedPartTcpListenRequest;

                    unsafe {
                        core::ptr::copy_nonoverlapping::<InlinedPartTcpListenRequest>(src, dst, 1);
                    }

                    // FIXME: Support multiple handles.
                    debug_assert!(
                        MessageInfo::from_raw(TcpListenRequest::MSGINFO.as_raw()).num_handles()
                            <= 1
                    );

                    // Don't call destructors on handles transferred to this buffer.
                    core::mem::forget(this);
                }

                do_serialize(self, buffer)
            }
        }

        impl MessageDeserialize for TcpListenRequest {
            type Reader<'a> = TcpListenRequestReader<'a>;

            fn deserialize<'a>(
                buffer: &'a MessageBuffer,
                msginfo: MessageInfo,
            ) -> Option<TcpListenRequestReader<'a>> {
                if msginfo == Self::MSGINFO {
                    Some(TcpListenRequestReader { buffer })
                } else {
                    None
                }
            }
        }

        pub struct TcpListenRequestReader<'a> {
            #[allow(dead_code)]
            buffer: &'a MessageBuffer,
        }

        impl<'a> TcpListenRequestReader<'a> {
            #[allow(dead_code)]
            fn as_ref(&self, buffer: &'a MessageBuffer) -> &'a InlinedPartTcpListenRequest {
                unsafe { &*(buffer as *const _ as *const InlinedPartTcpListenRequest) }
            }

            pub fn port(&self) -> u16 {
                let m = self.as_ref(self.buffer);
                m.port
            }
        }

        #[repr(C)]
        pub struct TcpListenReply {}

        #[repr(C)]
        struct InlinedPartTcpListenReply {}

        impl MessageSerialize for TcpListenReply {
            const MSGINFO: MessageInfo = MessageInfo::from_raw(
                (11 << 14) | (0 << 12) | ::core::mem::size_of::<TcpListenReply>() as isize,
            );

            fn serialize(self, buffer: &mut MessageBuffer) {
                // TODO: Make this a compile-time assertion.
                debug_assert!(::core::mem::size_of::<TcpListenReply>() < 1 << 12);

                // The actual serialization is done in this const fn. This is to
                // ensure the serialization can be done with const operations.
                const fn do_serialize(this: TcpListenReply, buffer: &mut MessageBuffer) {
                    let object = InlinedPartTcpListenReply {};

                    let dst = buffer as *mut _ as *mut InlinedPartTcpListenReply;
                    let src = &object as *const _ as *const InlinedPartTcpListenReply;

                    unsafe {
                        core::ptr::copy_nonoverlapping::<InlinedPartTcpListenReply>(src, dst, 1);
                    }

                    // FIXME: Support multiple handles.
                    debug_assert!(
                        MessageInfo::from_raw(TcpListenReply::MSGINFO.as_raw()).num_handles() <= 1
                    );

                    // Don't call destructors on handles transferred to this buffer.
                    core::mem::forget(this);
                }

                do_serialize(self, buffer)
            }
        }

        impl MessageDeserialize for TcpListenReply {
            type Reader<'a> = TcpListenReplyReader<'a>;

            fn deserialize<'a>(
                buffer: &'a MessageBuffer,
                msginfo: MessageInfo,
            ) -> Option<TcpListenReplyReader<'a>> {
                if msginfo == Self::MSGINFO {
                    Some(TcpListenReplyReader { buffer })
                } else {
                    None
                }
            }
        }

        pub struct TcpListenReplyReader<'a> {
            #[allow(dead_code)]
            buffer: &'a MessageBuffer,
        }

        impl<'a> TcpListenReplyReader<'a> {
            #[allow(dead_code)]
            fn as_ref(&self, buffer: &'a MessageBuffer) -> &'a InlinedPartTcpListenReply {
                unsafe { &*(buffer as *const _ as *const InlinedPartTcpListenReply) }
            }
        }

        #[repr(C)]
        pub struct TcpSendRequest {
            pub data: ftl_types::idl::BytesField<2048>,
        }

        #[repr(C)]
        struct InlinedPartTcpSendRequest {
            pub data: ftl_types::idl::BytesField<2048>,
        }

        impl MessageSerialize for TcpSendRequest {
            const MSGINFO: MessageInfo = MessageInfo::from_raw(
                (12 << 14) | (0 << 12) | ::core::mem::size_of::<TcpSendRequest>() as isize,
            );

            fn serialize(self, buffer: &mut MessageBuffer) {
                // TODO: Make this a compile-time assertion.
                debug_assert!(::core::mem::size_of::<TcpSendRequest>() < 1 << 12);

                // The actual serialization is done in this const fn. This is to
                // ensure the serialization can be done with const operations.
                const fn do_serialize(this: TcpSendRequest, buffer: &mut MessageBuffer) {
                    let object = InlinedPartTcpSendRequest { data: this.data };

                    let dst = buffer as *mut _ as *mut InlinedPartTcpSendRequest;
                    let src = &object as *const _ as *const InlinedPartTcpSendRequest;

                    unsafe {
                        core::ptr::copy_nonoverlapping::<InlinedPartTcpSendRequest>(src, dst, 1);
                    }

                    // FIXME: Support multiple handles.
                    debug_assert!(
                        MessageInfo::from_raw(TcpSendRequest::MSGINFO.as_raw()).num_handles() <= 1
                    );

                    // Don't call destructors on handles transferred to this buffer.
                    core::mem::forget(this);
                }

                do_serialize(self, buffer)
            }
        }

        impl MessageDeserialize for TcpSendRequest {
            type Reader<'a> = TcpSendRequestReader<'a>;

            fn deserialize<'a>(
                buffer: &'a MessageBuffer,
                msginfo: MessageInfo,
            ) -> Option<TcpSendRequestReader<'a>> {
                if msginfo == Self::MSGINFO {
                    Some(TcpSendRequestReader { buffer })
                } else {
                    None
                }
            }
        }

        pub struct TcpSendRequestReader<'a> {
            #[allow(dead_code)]
            buffer: &'a MessageBuffer,
        }

        impl<'a> TcpSendRequestReader<'a> {
            #[allow(dead_code)]
            fn as_ref(&self, buffer: &'a MessageBuffer) -> &'a InlinedPartTcpSendRequest {
                unsafe { &*(buffer as *const _ as *const InlinedPartTcpSendRequest) }
            }

            pub fn data(&self) -> ftl_types::idl::BytesField<2048> {
                let m = self.as_ref(self.buffer);
                m.data
            }
        }

        #[repr(C)]
        pub struct TcpSendReply {}

        #[repr(C)]
        struct InlinedPartTcpSendReply {}

        impl MessageSerialize for TcpSendReply {
            const MSGINFO: MessageInfo = MessageInfo::from_raw(
                (13 << 14) | (0 << 12) | ::core::mem::size_of::<TcpSendReply>() as isize,
            );

            fn serialize(self, buffer: &mut MessageBuffer) {
                // TODO: Make this a compile-time assertion.
                debug_assert!(::core::mem::size_of::<TcpSendReply>() < 1 << 12);

                // The actual serialization is done in this const fn. This is to
                // ensure the serialization can be done with const operations.
                const fn do_serialize(this: TcpSendReply, buffer: &mut MessageBuffer) {
                    let object = InlinedPartTcpSendReply {};

                    let dst = buffer as *mut _ as *mut InlinedPartTcpSendReply;
                    let src = &object as *const _ as *const InlinedPartTcpSendReply;

                    unsafe {
                        core::ptr::copy_nonoverlapping::<InlinedPartTcpSendReply>(src, dst, 1);
                    }

                    // FIXME: Support multiple handles.
                    debug_assert!(
                        MessageInfo::from_raw(TcpSendReply::MSGINFO.as_raw()).num_handles() <= 1
                    );

                    // Don't call destructors on handles transferred to this buffer.
                    core::mem::forget(this);
                }

                do_serialize(self, buffer)
            }
        }

        impl MessageDeserialize for TcpSendReply {
            type Reader<'a> = TcpSendReplyReader<'a>;

            fn deserialize<'a>(
                buffer: &'a MessageBuffer,
                msginfo: MessageInfo,
            ) -> Option<TcpSendReplyReader<'a>> {
                if msginfo == Self::MSGINFO {
                    Some(TcpSendReplyReader { buffer })
                } else {
                    None
                }
            }
        }

        pub struct TcpSendReplyReader<'a> {
            #[allow(dead_code)]
            buffer: &'a MessageBuffer,
        }

        impl<'a> TcpSendReplyReader<'a> {
            #[allow(dead_code)]
            fn as_ref(&self, buffer: &'a MessageBuffer) -> &'a InlinedPartTcpSendReply {
                unsafe { &*(buffer as *const _ as *const InlinedPartTcpSendReply) }
            }
        }
    }
}

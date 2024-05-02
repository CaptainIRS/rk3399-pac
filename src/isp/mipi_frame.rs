#[doc = "Register `MIPI_FRAME` reader"]
pub type R = crate::R<MipiFrameSpec>;
#[doc = "Field `frame_number_fs` reader - 16 bit frame number from Frame Start (FS) short\n\npacket"]
pub type FrameNumberFsR = crate::FieldReader<u16>;
#[doc = "Field `frame_number_fe` reader - 16 bit frame number from Frame End (FE) short\n\npacket"]
pub type FrameNumberFeR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 16 bit frame number from Frame Start (FS) short\n\npacket"]
    #[inline(always)]
    pub fn frame_number_fs(&self) -> FrameNumberFsR {
        FrameNumberFsR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 16 bit frame number from Frame End (FE) short\n\npacket"]
    #[inline(always)]
    pub fn frame_number_fe(&self) -> FrameNumberFeR {
        FrameNumberFeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "frame number from frame start and frame end short packets\n\nNote: This register is only available in MIPI interface version 2 of ISP \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_frame::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiFrameSpec;
impl crate::RegisterSpec for MipiFrameSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_frame::R`](R) reader structure"]
impl crate::Readable for MipiFrameSpec {}
#[doc = "`reset()` method sets MIPI_FRAME to value 0"]
impl crate::Resettable for MipiFrameSpec {
    const RESET_VALUE: u32 = 0;
}

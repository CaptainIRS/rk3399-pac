#[doc = "Register `FRAME_COUNT` reader"]
pub type R = crate::R<FrameCountSpec>;
#[doc = "Field `frame_counter` reader - Current frame count of processing"]
pub type FrameCounterR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Current frame count of processing"]
    #[inline(always)]
    pub fn frame_counter(&self) -> FrameCounterR {
        FrameCounterR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Frame counter\n\nNote: In the ISP_FRAME_COUNT register the number of processed frames are displayed. \n\nFor example: If a 8 is programmed into the ISP_ACQ_NR_FRAMES register, a read access to \n\nthe ISP_FRAME_COUNT register during processing of the first picture shows a 7. \n\n\n\nAfter the entire frames are processed the ISP_OFF interrupt is generated and the \n\nISP_FRAME_COUNT has the count zero. In case a '0' is programmed into the \n\nISP_ACQ_NR_FRAMES register (continues mode) the ISP_FRAME_COUNT register keeps the \n\nvalue '0'. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frame_count::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrameCountSpec;
impl crate::RegisterSpec for FrameCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frame_count::R`](R) reader structure"]
impl crate::Readable for FrameCountSpec {}
#[doc = "`reset()` method sets FRAME_COUNT to value 0"]
impl crate::Resettable for FrameCountSpec {
    const RESET_VALUE: u32 = 0;
}

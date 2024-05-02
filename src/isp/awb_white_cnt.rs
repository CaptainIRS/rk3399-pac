#[doc = "Register `AWB_WHITE_CNT` reader"]
pub type R = crate::R<AwbWhiteCntSpec>;
#[doc = "Field `AWB_WHITE_CNT` reader - White pixel count, number of 'white pixels' found\n\nduring last measurement, i.e. pixels included in mean\n\nvalue calculation"]
pub type AwbWhiteCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:25 - White pixel count, number of 'white pixels' found\n\nduring last measurement, i.e. pixels included in mean\n\nvalue calculation"]
    #[inline(always)]
    pub fn awb_white_cnt(&self) -> AwbWhiteCntR {
        AwbWhiteCntR::new(self.bits & 0x03ff_ffff)
    }
}
#[doc = "Auto white balance white pixel count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awb_white_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AwbWhiteCntSpec;
impl crate::RegisterSpec for AwbWhiteCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awb_white_cnt::R`](R) reader structure"]
impl crate::Readable for AwbWhiteCntSpec {}
#[doc = "`reset()` method sets AWB_WHITE_CNT to value 0"]
impl crate::Resettable for AwbWhiteCntSpec {
    const RESET_VALUE: u32 = 0;
}

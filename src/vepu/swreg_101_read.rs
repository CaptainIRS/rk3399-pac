#[doc = "Register `SWREG_101_READ` reader"]
pub type R = crate::R<Swreg101ReadSpec>;
#[doc = "Field `MAX_VID_WIDTH` reader - Field0000 Description"]
pub type MaxVidWidthR = crate::FieldReader<u16>;
#[doc = "Field `HW_CONFIG` reader - Field0000 Description"]
pub type HwConfigR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:11 - Field0000 Description"]
    #[inline(always)]
    pub fn max_vid_width(&self) -> MaxVidWidthR {
        MaxVidWidthR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:31 - Field0000 Description"]
    #[inline(always)]
    pub fn hw_config(&self) -> HwConfigR {
        HwConfigR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
#[doc = "hw config reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_101_read::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg101ReadSpec;
impl crate::RegisterSpec for Swreg101ReadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_101_read::R`](R) reader structure"]
impl crate::Readable for Swreg101ReadSpec {}
#[doc = "`reset()` method sets SWREG_101_READ to value 0x1f52_2780"]
impl crate::Resettable for Swreg101ReadSpec {
    const RESET_VALUE: u32 = 0x1f52_2780;
}

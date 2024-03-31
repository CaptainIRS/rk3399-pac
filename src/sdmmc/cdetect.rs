#[doc = "Register `CDETECT` reader"]
pub type R = crate::R<CdetectSpec>;
#[doc = "Field `CARD_DETECT_N` reader - Value on card_detect_n input ports; read-only bits. 0 represents\n\npresence of card."]
pub type CardDetectNR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Value on card_detect_n input ports; read-only bits. 0 represents\n\npresence of card."]
    #[inline(always)]
    pub fn card_detect_n(&self) -> CardDetectNR {
        CardDetectNR::new((self.bits & 1) != 0)
    }
}
#[doc = "Card-detect register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdetect::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CdetectSpec;
impl crate::RegisterSpec for CdetectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdetect::R`](R) reader structure"]
impl crate::Readable for CdetectSpec {}
#[doc = "`reset()` method sets CDETECT to value 0"]
impl crate::Resettable for CdetectSpec {
    const RESET_VALUE: u32 = 0;
}

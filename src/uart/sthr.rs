#[doc = "Register `STHR` reader"]
pub type R = crate::R<SthrSpec>;
#[doc = "Field `SHADOW_THR` reader - This is a shadow register for the THR."]
pub type ShadowThrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - This is a shadow register for the THR."]
    #[inline(always)]
    pub fn shadow_thr(&self) -> ShadowThrR {
        ShadowThrR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Shadow Transmit Holding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sthr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SthrSpec;
impl crate::RegisterSpec for SthrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sthr::R`](R) reader structure"]
impl crate::Readable for SthrSpec {}
#[doc = "`reset()` method sets STHR to value 0"]
impl crate::Resettable for SthrSpec {
    const RESET_VALUE: u32 = 0;
}

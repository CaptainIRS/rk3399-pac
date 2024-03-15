#[doc = "Register `DMAC_LC0_%s` reader"]
pub type R = crate::R<DmacLc0_Spec>;
#[doc = "Field `DMAC_LC0__BITS_1` reader - Loop counter 0 iterations"]
pub type DmacLc0_Bits1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Loop counter 0 iterations"]
    #[inline(always)]
    pub fn dmac_lc0__bits_1(&self) -> DmacLc0_Bits1R {
        DmacLc0_Bits1R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Loop Counter 0 Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_lc0_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacLc0_Spec;
impl crate::RegisterSpec for DmacLc0_Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_lc0_::R`](R) reader structure"]
impl crate::Readable for DmacLc0_Spec {}
#[doc = "`reset()` method sets DMAC_LC0_%s to value 0"]
impl crate::Resettable for DmacLc0_Spec {
    const RESET_VALUE: u32 = 0;
}

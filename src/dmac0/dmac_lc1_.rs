#[doc = "Register `DMAC_LC1_%s` reader"]
pub type R = crate::R<DmacLc1_Spec>;
#[doc = "Field `DMAC_LC1__BITS_1` reader - Loop counter 1 iterations"]
pub type DmacLc1_Bits1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Loop counter 1 iterations"]
    #[inline(always)]
    pub fn dmac_lc1__bits_1(&self) -> DmacLc1_Bits1R {
        DmacLc1_Bits1R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Loop Counter 1 Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_lc1_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacLc1_Spec;
impl crate::RegisterSpec for DmacLc1_Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_lc1_::R`](R) reader structure"]
impl crate::Readable for DmacLc1_Spec {}
#[doc = "`reset()` method sets DMAC_LC1_%s to value 0"]
impl crate::Resettable for DmacLc1_Spec {
    const RESET_VALUE: u32 = 0;
}

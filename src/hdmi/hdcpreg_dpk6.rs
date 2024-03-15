#[doc = "Register `HDCPREG_DPK6` writer"]
pub type W = crate::W<HdcpregDpk6Spec>;
#[doc = "Field `DPK_DATA` writer - Contains the value of DPK\\[x\\]\\[55:48\\]"]
pub type DpkDataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Contains the value of DPK\\[x\\]\\[55:48\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dpk_data(&mut self) -> DpkDataW<HdcpregDpk6Spec> {
        DpkDataW::new(self, 0)
    }
}
#[doc = "Contains the value of DPK\\[x\\]\\[55:48\\]\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_dpk6::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcpregDpk6Spec;
impl crate::RegisterSpec for HdcpregDpk6Spec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`hdcpreg_dpk6::W`](W) writer structure"]
impl crate::Writable for HdcpregDpk6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HDCPREG_DPK6 to value 0"]
impl crate::Resettable for HdcpregDpk6Spec {
    const RESET_VALUE: u8 = 0;
}

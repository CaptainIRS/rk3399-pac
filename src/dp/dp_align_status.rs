#[doc = "Register `DP_ALIGN_STATUS` reader"]
pub type R = crate::R<DpAlignStatusSpec>;
#[doc = "Register `DP_ALIGN_STATUS` writer"]
pub type W = crate::W<DpAlignStatusSpec>;
#[doc = "Field `DP_ALIGN_STATUS` reader - DP_ALIGN_STATUS"]
pub type DpAlignStatusR = crate::FieldReader;
#[doc = "Field `DP_ALIGN_STATUS` writer - DP_ALIGN_STATUS"]
pub type DpAlignStatusW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DP_ALIGN_STATUS"]
    #[inline(always)]
    pub fn dp_align_status(&self) -> DpAlignStatusR {
        DpAlignStatusR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DP_ALIGN_STATUS"]
    #[inline(always)]
    #[must_use]
    pub fn dp_align_status(&mut self) -> DpAlignStatusW<DpAlignStatusSpec> {
        DpAlignStatusW::new(self, 0)
    }
}
#[doc = "DP Align Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_align_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_align_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpAlignStatusSpec;
impl crate::RegisterSpec for DpAlignStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_align_status::R`](R) reader structure"]
impl crate::Readable for DpAlignStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`dp_align_status::W`](W) writer structure"]
impl crate::Writable for DpAlignStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DP_ALIGN_STATUS to value 0"]
impl crate::Resettable for DpAlignStatusSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `SPDIF_VLDFRn` reader"]
pub type R = crate::R<SpdifVldfrnSpec>;
#[doc = "Register `SPDIF_VLDFRn` writer"]
pub type W = crate::W<SpdifVldfrnSpec>;
#[doc = "Field `VLDFR_SUB_0` reader - Validity Flag Subframe 0 Validity Flag for Subframe 0"]
pub type VldfrSub0R = crate::FieldReader<u16>;
#[doc = "Field `VLDFR_SUB_0` writer - Validity Flag Subframe 0 Validity Flag for Subframe 0"]
pub type VldfrSub0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `VLDFR_SUB_1` reader - Validity Flag Subframe 1 Validity Flag Register 0"]
pub type VldfrSub1R = crate::FieldReader<u16>;
#[doc = "Field `VLDFR_SUB_1` writer - Validity Flag Subframe 1 Validity Flag Register 0"]
pub type VldfrSub1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Validity Flag Subframe 0 Validity Flag for Subframe 0"]
    #[inline(always)]
    pub fn vldfr_sub_0(&self) -> VldfrSub0R {
        VldfrSub0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Validity Flag Subframe 1 Validity Flag Register 0"]
    #[inline(always)]
    pub fn vldfr_sub_1(&self) -> VldfrSub1R {
        VldfrSub1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Validity Flag Subframe 0 Validity Flag for Subframe 0"]
    #[inline(always)]
    #[must_use]
    pub fn vldfr_sub_0(&mut self) -> VldfrSub0W<SpdifVldfrnSpec> {
        VldfrSub0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Validity Flag Subframe 1 Validity Flag Register 0"]
    #[inline(always)]
    #[must_use]
    pub fn vldfr_sub_1(&mut self) -> VldfrSub1W<SpdifVldfrnSpec> {
        VldfrSub1W::new(self, 16)
    }
}
#[doc = "Validity Flag Register n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdif_vldfrn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdif_vldfrn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdifVldfrnSpec;
impl crate::RegisterSpec for SpdifVldfrnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdif_vldfrn::R`](R) reader structure"]
impl crate::Readable for SpdifVldfrnSpec {}
#[doc = "`write(|w| ..)` method takes [`spdif_vldfrn::W`](W) writer structure"]
impl crate::Writable for SpdifVldfrnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPDIF_VLDFRn to value 0"]
impl crate::Resettable for SpdifVldfrnSpec {
    const RESET_VALUE: u32 = 0;
}

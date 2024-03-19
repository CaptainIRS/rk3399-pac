#[doc = "Register `CRU_GLB_SRST_FST_VALUE` reader"]
pub type R = crate::R<CruGlbSrstFstValueSpec>;
#[doc = "Register `CRU_GLB_SRST_FST_VALUE` writer"]
pub type W = crate::W<CruGlbSrstFstValueSpec>;
#[doc = "Field `GLB_SRST_FST_VALUE` reader - The first global software reset config value\n\nIf config 0xfdb9, it will generate first global software reset"]
pub type GlbSrstFstValueR = crate::FieldReader<u16>;
#[doc = "Field `GLB_SRST_FST_VALUE` writer - The first global software reset config value\n\nIf config 0xfdb9, it will generate first global software reset"]
pub type GlbSrstFstValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The first global software reset config value\n\nIf config 0xfdb9, it will generate first global software reset"]
    #[inline(always)]
    pub fn glb_srst_fst_value(&self) -> GlbSrstFstValueR {
        GlbSrstFstValueR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The first global software reset config value\n\nIf config 0xfdb9, it will generate first global software reset"]
    #[inline(always)]
    #[must_use]
    pub fn glb_srst_fst_value(&mut self) -> GlbSrstFstValueW<CruGlbSrstFstValueSpec> {
        GlbSrstFstValueW::new(self, 0)
    }
}
#[doc = "The first global software reset config value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_glb_srst_fst_value::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_glb_srst_fst_value::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruGlbSrstFstValueSpec;
impl crate::RegisterSpec for CruGlbSrstFstValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_glb_srst_fst_value::R`](R) reader structure"]
impl crate::Readable for CruGlbSrstFstValueSpec {}
#[doc = "`write(|w| ..)` method takes [`cru_glb_srst_fst_value::W`](W) writer structure"]
impl crate::Writable for CruGlbSrstFstValueSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_GLB_SRST_FST_VALUE to value 0"]
impl crate::Resettable for CruGlbSrstFstValueSpec {
    const RESET_VALUE: u32 = 0;
}

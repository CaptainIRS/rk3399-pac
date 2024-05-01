#[doc = "Register `SWREG30_VP9_GREF_SCALE` reader"]
pub type R = crate::R<Swreg30Vp9GrefScaleSpec>;
#[doc = "Register `SWREG30_VP9_GREF_SCALE` writer"]
pub type W = crate::W<Swreg30Vp9GrefScaleSpec>;
#[doc = "Field `SW_VP9_GREF_HOR_SCALE` reader - horizontal scaling factor for golden reference picture\n\nhorizontal scaling factor for golden reference picture\n\nsw_vp9_gref_hor_scale = (golden_ref_width / cur_width) *\n\n0x4000"]
pub type SwVp9GrefHorScaleR = crate::FieldReader<u16>;
#[doc = "Field `SW_VP9_GREF_HOR_SCALE` writer - horizontal scaling factor for golden reference picture\n\nhorizontal scaling factor for golden reference picture\n\nsw_vp9_gref_hor_scale = (golden_ref_width / cur_width) *\n\n0x4000"]
pub type SwVp9GrefHorScaleW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SW_VP9_GREF_VER_SCALE` reader - vertical scaling factor for golden reference picture\n\nvertical scaling factor for golden reference picture"]
pub type SwVp9GrefVerScaleR = crate::FieldReader<u16>;
#[doc = "Field `SW_VP9_GREF_VER_SCALE` writer - vertical scaling factor for golden reference picture\n\nvertical scaling factor for golden reference picture"]
pub type SwVp9GrefVerScaleW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - horizontal scaling factor for golden reference picture\n\nhorizontal scaling factor for golden reference picture\n\nsw_vp9_gref_hor_scale = (golden_ref_width / cur_width) *\n\n0x4000"]
    #[inline(always)]
    pub fn sw_vp9_gref_hor_scale(&self) -> SwVp9GrefHorScaleR {
        SwVp9GrefHorScaleR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - vertical scaling factor for golden reference picture\n\nvertical scaling factor for golden reference picture"]
    #[inline(always)]
    pub fn sw_vp9_gref_ver_scale(&self) -> SwVp9GrefVerScaleR {
        SwVp9GrefVerScaleR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - horizontal scaling factor for golden reference picture\n\nhorizontal scaling factor for golden reference picture\n\nsw_vp9_gref_hor_scale = (golden_ref_width / cur_width) *\n\n0x4000"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9_gref_hor_scale(&mut self) -> SwVp9GrefHorScaleW<Swreg30Vp9GrefScaleSpec> {
        SwVp9GrefHorScaleW::new(self, 0)
    }
    #[doc = "Bits 16:31 - vertical scaling factor for golden reference picture\n\nvertical scaling factor for golden reference picture"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9_gref_ver_scale(&mut self) -> SwVp9GrefVerScaleW<Swreg30Vp9GrefScaleSpec> {
        SwVp9GrefVerScaleW::new(self, 16)
    }
}
#[doc = "scaling factor for golden reference picture\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg30_vp9_gref_scale::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg30_vp9_gref_scale::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg30Vp9GrefScaleSpec;
impl crate::RegisterSpec for Swreg30Vp9GrefScaleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg30_vp9_gref_scale::R`](R) reader structure"]
impl crate::Readable for Swreg30Vp9GrefScaleSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg30_vp9_gref_scale::W`](W) writer structure"]
impl crate::Writable for Swreg30Vp9GrefScaleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG30_VP9_GREF_SCALE to value 0"]
impl crate::Resettable for Swreg30Vp9GrefScaleSpec {
    const RESET_VALUE: u32 = 0;
}

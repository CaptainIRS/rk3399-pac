#[doc = "Register `SWREG31_VP9_AREF_SCALE` reader"]
pub type R = crate::R<Swreg31Vp9ArefScaleSpec>;
#[doc = "Register `SWREG31_VP9_AREF_SCALE` writer"]
pub type W = crate::W<Swreg31Vp9ArefScaleSpec>;
#[doc = "Field `SW_VP9_AREF_HOR_SCALE` reader - horizontal scaling factor for alfter reference picture\n\nhorizontal scaling factor for alfter reference picture\n\nsw_vp9_gref_hor_scale = (alfter_ref_width / cur_width) * 0x4000"]
pub type SwVp9ArefHorScaleR = crate::FieldReader<u16>;
#[doc = "Field `SW_VP9_AREF_HOR_SCALE` writer - horizontal scaling factor for alfter reference picture\n\nhorizontal scaling factor for alfter reference picture\n\nsw_vp9_gref_hor_scale = (alfter_ref_width / cur_width) * 0x4000"]
pub type SwVp9ArefHorScaleW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SW_VP9_AREF_VER_SCALE` reader - vertical scaling factor for alfter reference picture\n\nvertical scaling factor for alfter reference picture"]
pub type SwVp9ArefVerScaleR = crate::FieldReader<u16>;
#[doc = "Field `SW_VP9_AREF_VER_SCALE` writer - vertical scaling factor for alfter reference picture\n\nvertical scaling factor for alfter reference picture"]
pub type SwVp9ArefVerScaleW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - horizontal scaling factor for alfter reference picture\n\nhorizontal scaling factor for alfter reference picture\n\nsw_vp9_gref_hor_scale = (alfter_ref_width / cur_width) * 0x4000"]
    #[inline(always)]
    pub fn sw_vp9_aref_hor_scale(&self) -> SwVp9ArefHorScaleR {
        SwVp9ArefHorScaleR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - vertical scaling factor for alfter reference picture\n\nvertical scaling factor for alfter reference picture"]
    #[inline(always)]
    pub fn sw_vp9_aref_ver_scale(&self) -> SwVp9ArefVerScaleR {
        SwVp9ArefVerScaleR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - horizontal scaling factor for alfter reference picture\n\nhorizontal scaling factor for alfter reference picture\n\nsw_vp9_gref_hor_scale = (alfter_ref_width / cur_width) * 0x4000"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9_aref_hor_scale(&mut self) -> SwVp9ArefHorScaleW<Swreg31Vp9ArefScaleSpec> {
        SwVp9ArefHorScaleW::new(self, 0)
    }
    #[doc = "Bits 16:31 - vertical scaling factor for alfter reference picture\n\nvertical scaling factor for alfter reference picture"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9_aref_ver_scale(&mut self) -> SwVp9ArefVerScaleW<Swreg31Vp9ArefScaleSpec> {
        SwVp9ArefVerScaleW::new(self, 16)
    }
}
#[doc = "scaling factor for alfter reference picture\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg31_vp9_aref_scale::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg31_vp9_aref_scale::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg31Vp9ArefScaleSpec;
impl crate::RegisterSpec for Swreg31Vp9ArefScaleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg31_vp9_aref_scale::R`](R) reader structure"]
impl crate::Readable for Swreg31Vp9ArefScaleSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg31_vp9_aref_scale::W`](W) writer structure"]
impl crate::Writable for Swreg31Vp9ArefScaleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG31_VP9_AREF_SCALE to value 0"]
impl crate::Resettable for Swreg31Vp9ArefScaleSpec {
    const RESET_VALUE: u32 = 0;
}

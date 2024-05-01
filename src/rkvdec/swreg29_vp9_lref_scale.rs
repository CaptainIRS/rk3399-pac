#[doc = "Register `SWREG29_VP9_LREF_SCALE` reader"]
pub type R = crate::R<Swreg29Vp9LrefScaleSpec>;
#[doc = "Register `SWREG29_VP9_LREF_SCALE` writer"]
pub type W = crate::W<Swreg29Vp9LrefScaleSpec>;
#[doc = "Field `SW_VP9_LREF_HOR_SCALE` reader - horizontal scaling factor for last reference picture\n\nhorizontal scaling factor for last reference picture\n\nsw_vp9_lref_hor_scale = (last_ref_width / cur_width) * 0x4000"]
pub type SwVp9LrefHorScaleR = crate::FieldReader<u16>;
#[doc = "Field `SW_VP9_LREF_HOR_SCALE` writer - horizontal scaling factor for last reference picture\n\nhorizontal scaling factor for last reference picture\n\nsw_vp9_lref_hor_scale = (last_ref_width / cur_width) * 0x4000"]
pub type SwVp9LrefHorScaleW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SW_VP9_LREF_VER_SCALE` reader - vertical scaling factor for last reference picture\n\nvertical scaling factor for last reference picture"]
pub type SwVp9LrefVerScaleR = crate::FieldReader<u16>;
#[doc = "Field `SW_VP9_LREF_VER_SCALE` writer - vertical scaling factor for last reference picture\n\nvertical scaling factor for last reference picture"]
pub type SwVp9LrefVerScaleW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - horizontal scaling factor for last reference picture\n\nhorizontal scaling factor for last reference picture\n\nsw_vp9_lref_hor_scale = (last_ref_width / cur_width) * 0x4000"]
    #[inline(always)]
    pub fn sw_vp9_lref_hor_scale(&self) -> SwVp9LrefHorScaleR {
        SwVp9LrefHorScaleR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - vertical scaling factor for last reference picture\n\nvertical scaling factor for last reference picture"]
    #[inline(always)]
    pub fn sw_vp9_lref_ver_scale(&self) -> SwVp9LrefVerScaleR {
        SwVp9LrefVerScaleR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - horizontal scaling factor for last reference picture\n\nhorizontal scaling factor for last reference picture\n\nsw_vp9_lref_hor_scale = (last_ref_width / cur_width) * 0x4000"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9_lref_hor_scale(&mut self) -> SwVp9LrefHorScaleW<Swreg29Vp9LrefScaleSpec> {
        SwVp9LrefHorScaleW::new(self, 0)
    }
    #[doc = "Bits 16:31 - vertical scaling factor for last reference picture\n\nvertical scaling factor for last reference picture"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9_lref_ver_scale(&mut self) -> SwVp9LrefVerScaleW<Swreg29Vp9LrefScaleSpec> {
        SwVp9LrefVerScaleW::new(self, 16)
    }
}
#[doc = "scaling factor for last reference picture\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg29_vp9_lref_scale::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg29_vp9_lref_scale::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg29Vp9LrefScaleSpec;
impl crate::RegisterSpec for Swreg29Vp9LrefScaleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg29_vp9_lref_scale::R`](R) reader structure"]
impl crate::Readable for Swreg29Vp9LrefScaleSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg29_vp9_lref_scale::W`](W) writer structure"]
impl crate::Writable for Swreg29Vp9LrefScaleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG29_VP9_LREF_SCALE to value 0"]
impl crate::Resettable for Swreg29Vp9LrefScaleSpec {
    const RESET_VALUE: u32 = 0;
}

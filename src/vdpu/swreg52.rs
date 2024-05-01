#[doc = "Register `SWREG52` reader"]
pub type R = crate::R<Swreg52Spec>;
#[doc = "Register `SWREG52` writer"]
pub type W = crate::W<Swreg52Spec>;
#[doc = "Field `SW_YDIM_MBST` reader - the Y dimension value for Start MB from SW\n\nit may be used in error concealment case"]
pub type SwYdimMbstR = crate::FieldReader;
#[doc = "Field `SW_YDIM_MBST` writer - the Y dimension value for Start MB from SW\n\nit may be used in error concealment case"]
pub type SwYdimMbstW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SW_XDIM_MBST` reader - the X dimension value for Start MB from SW\n\nit may be used in error concealment case"]
pub type SwXdimMbstR = crate::FieldReader<u16>;
#[doc = "Field `SW_XDIM_MBST` writer - the X dimension value for Start MB from SW\n\nit may be used in error concealment case"]
pub type SwXdimMbstW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `SW_ADV_PREF_THRD` reader - the threshold value for advanced prefetch\n\nwhen current MB num > this threshold value,then advanced mode\n\nwill be closed"]
pub type SwAdvPrefThrdR = crate::FieldReader<u16>;
#[doc = "Field `SW_ADV_PREF_THRD` writer - the threshold value for advanced prefetch\n\nwhen current MB num > this threshold value,then advanced mode\n\nwill be closed"]
pub type SwAdvPrefThrdW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:7 - the Y dimension value for Start MB from SW\n\nit may be used in error concealment case"]
    #[inline(always)]
    pub fn sw_ydim_mbst(&self) -> SwYdimMbstR {
        SwYdimMbstR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:16 - the X dimension value for Start MB from SW\n\nit may be used in error concealment case"]
    #[inline(always)]
    pub fn sw_xdim_mbst(&self) -> SwXdimMbstR {
        SwXdimMbstR::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bits 17:30 - the threshold value for advanced prefetch\n\nwhen current MB num > this threshold value,then advanced mode\n\nwill be closed"]
    #[inline(always)]
    pub fn sw_adv_pref_thrd(&self) -> SwAdvPrefThrdR {
        SwAdvPrefThrdR::new(((self.bits >> 17) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - the Y dimension value for Start MB from SW\n\nit may be used in error concealment case"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ydim_mbst(&mut self) -> SwYdimMbstW<Swreg52Spec> {
        SwYdimMbstW::new(self, 0)
    }
    #[doc = "Bits 8:16 - the X dimension value for Start MB from SW\n\nit may be used in error concealment case"]
    #[inline(always)]
    #[must_use]
    pub fn sw_xdim_mbst(&mut self) -> SwXdimMbstW<Swreg52Spec> {
        SwXdimMbstW::new(self, 8)
    }
    #[doc = "Bits 17:30 - the threshold value for advanced prefetch\n\nwhen current MB num > this threshold value,then advanced mode\n\nwill be closed"]
    #[inline(always)]
    #[must_use]
    pub fn sw_adv_pref_thrd(&mut self) -> SwAdvPrefThrdW<Swreg52Spec> {
        SwAdvPrefThrdW::new(self, 17)
    }
}
#[doc = "error concealment case related\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg52::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg52::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg52Spec;
impl crate::RegisterSpec for Swreg52Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg52::R`](R) reader structure"]
impl crate::Readable for Swreg52Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg52::W`](W) writer structure"]
impl crate::Writable for Swreg52Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG52 to value 0"]
impl crate::Resettable for Swreg52Spec {
    const RESET_VALUE: u32 = 0;
}

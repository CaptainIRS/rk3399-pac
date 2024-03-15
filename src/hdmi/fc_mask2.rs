#[doc = "Register `FC_MASK2` reader"]
pub type R = crate::R<FcMask2Spec>;
#[doc = "Register `FC_MASK2` writer"]
pub type W = crate::W<FcMask2Spec>;
#[doc = "Field `HIGHPRIORITY_OVERFLOW` reader - Mask bit for FC_INT2.HighPriority_overflow interrupt bit"]
pub type HighpriorityOverflowR = crate::BitReader;
#[doc = "Field `HIGHPRIORITY_OVERFLOW` writer - Mask bit for FC_INT2.HighPriority_overflow interrupt bit"]
pub type HighpriorityOverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOWPRIORITY_OVERFLOW` reader - Mask bit for FC_INT2.LowPriority_overflow interrupt bit"]
pub type LowpriorityOverflowR = crate::BitReader;
#[doc = "Field `LOWPRIORITY_OVERFLOW` writer - Mask bit for FC_INT2.LowPriority_overflow interrupt bit"]
pub type LowpriorityOverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRM` reader - Mask bit for FC_INT2.DRM interrupt bit. Reset Value: (HDMI_TX_20== 1) ? 1 : 0"]
pub type DrmR = crate::BitReader;
#[doc = "Field `DRM` writer - Mask bit for FC_INT2.DRM interrupt bit. Reset Value: (HDMI_TX_20== 1) ? 1 : 0"]
pub type DrmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mask bit for FC_INT2.HighPriority_overflow interrupt bit"]
    #[inline(always)]
    pub fn highpriority_overflow(&self) -> HighpriorityOverflowR {
        HighpriorityOverflowR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask bit for FC_INT2.LowPriority_overflow interrupt bit"]
    #[inline(always)]
    pub fn lowpriority_overflow(&self) -> LowpriorityOverflowR {
        LowpriorityOverflowR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask bit for FC_INT2.DRM interrupt bit. Reset Value: (HDMI_TX_20== 1) ? 1 : 0"]
    #[inline(always)]
    pub fn drm(&self) -> DrmR {
        DrmR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for FC_INT2.HighPriority_overflow interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn highpriority_overflow(&mut self) -> HighpriorityOverflowW<FcMask2Spec> {
        HighpriorityOverflowW::new(self, 0)
    }
    #[doc = "Bit 1 - Mask bit for FC_INT2.LowPriority_overflow interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn lowpriority_overflow(&mut self) -> LowpriorityOverflowW<FcMask2Spec> {
        LowpriorityOverflowW::new(self, 1)
    }
    #[doc = "Bit 4 - Mask bit for FC_INT2.DRM interrupt bit. Reset Value: (HDMI_TX_20== 1) ? 1 : 0"]
    #[inline(always)]
    #[must_use]
    pub fn drm(&mut self) -> DrmW<FcMask2Spec> {
        DrmW::new(self, 4)
    }
}
#[doc = "Mask bit for FC_INT2.HighPriority_overflow interrupt bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_mask2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_mask2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcMask2Spec;
impl crate::RegisterSpec for FcMask2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_mask2::R`](R) reader structure"]
impl crate::Readable for FcMask2Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_mask2::W`](W) writer structure"]
impl crate::Writable for FcMask2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_MASK2 to value 0"]
impl crate::Resettable for FcMask2Spec {
    const RESET_VALUE: u8 = 0;
}

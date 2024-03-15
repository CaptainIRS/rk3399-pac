#[doc = "Register `IH_FC_STAT2` reader"]
pub type R = crate::R<IhFcStat2Spec>;
#[doc = "Register `IH_FC_STAT2` writer"]
pub type W = crate::W<IhFcStat2Spec>;
#[doc = "Field `HIGHPRIORITY_OVERFLOW` reader - Frame Composer high priority packet queue descriptor overflow indication"]
pub type HighpriorityOverflowR = crate::BitReader;
#[doc = "Field `HIGHPRIORITY_OVERFLOW` writer - Frame Composer high priority packet queue descriptor overflow indication"]
pub type HighpriorityOverflowW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LOWPRIORITY_OVERFLOW` reader - Frame Composer low priority packet queue descriptor overflow indication"]
pub type LowpriorityOverflowR = crate::BitReader;
#[doc = "Field `LOWPRIORITY_OVERFLOW` writer - Frame Composer low priority packet queue descriptor overflow indication"]
pub type LowpriorityOverflowW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DRM` reader - Active after successful transmission of an DRM packet"]
pub type DrmR = crate::BitReader;
#[doc = "Field `DRM` writer - Active after successful transmission of an DRM packet"]
pub type DrmW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Frame Composer high priority packet queue descriptor overflow indication"]
    #[inline(always)]
    pub fn highpriority_overflow(&self) -> HighpriorityOverflowR {
        HighpriorityOverflowR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame Composer low priority packet queue descriptor overflow indication"]
    #[inline(always)]
    pub fn lowpriority_overflow(&self) -> LowpriorityOverflowR {
        LowpriorityOverflowR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Active after successful transmission of an DRM packet"]
    #[inline(always)]
    pub fn drm(&self) -> DrmR {
        DrmR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frame Composer high priority packet queue descriptor overflow indication"]
    #[inline(always)]
    #[must_use]
    pub fn highpriority_overflow(&mut self) -> HighpriorityOverflowW<IhFcStat2Spec> {
        HighpriorityOverflowW::new(self, 0)
    }
    #[doc = "Bit 1 - Frame Composer low priority packet queue descriptor overflow indication"]
    #[inline(always)]
    #[must_use]
    pub fn lowpriority_overflow(&mut self) -> LowpriorityOverflowW<IhFcStat2Spec> {
        LowpriorityOverflowW::new(self, 1)
    }
    #[doc = "Bit 4 - Active after successful transmission of an DRM packet"]
    #[inline(always)]
    #[must_use]
    pub fn drm(&mut self) -> DrmW<IhFcStat2Spec> {
        DrmW::new(self, 4)
    }
}
#[doc = "Frame Composer high priority packet queue descriptor overflow indication\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_fc_stat2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_fc_stat2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IhFcStat2Spec;
impl crate::RegisterSpec for IhFcStat2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ih_fc_stat2::R`](R) reader structure"]
impl crate::Readable for IhFcStat2Spec {}
#[doc = "`write(|w| ..)` method takes [`ih_fc_stat2::W`](W) writer structure"]
impl crate::Writable for IhFcStat2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0x13;
}
#[doc = "`reset()` method sets IH_FC_STAT2 to value 0"]
impl crate::Resettable for IhFcStat2Spec {
    const RESET_VALUE: u8 = 0;
}

#[doc = "Register `IH_MUTE_FC_STAT2` reader"]
pub type R = crate::R<IhMuteFcStat2Spec>;
#[doc = "Register `IH_MUTE_FC_STAT2` writer"]
pub type W = crate::W<IhMuteFcStat2Spec>;
#[doc = "Field `HIGHPRIORITY_OVERFLOW` reader - When set to 1, mutes ih_fc_stat2\\[0\\]"]
pub type HighpriorityOverflowR = crate::BitReader;
#[doc = "Field `HIGHPRIORITY_OVERFLOW` writer - When set to 1, mutes ih_fc_stat2\\[0\\]"]
pub type HighpriorityOverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOWPRIORITY_OVERFLOW` reader - When set to 1, mutes ih_fc_stat2\\[1\\]"]
pub type LowpriorityOverflowR = crate::BitReader;
#[doc = "Field `LOWPRIORITY_OVERFLOW` writer - When set to 1, mutes ih_fc_stat2\\[1\\]"]
pub type LowpriorityOverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRM` reader - When set to 1, mutes ih_fc_stat2\\[4\\]. Reset Value: (HDMI_TX_20== 1) ? 1 : 0"]
pub type DrmR = crate::BitReader;
#[doc = "Field `DRM` writer - When set to 1, mutes ih_fc_stat2\\[4\\]. Reset Value: (HDMI_TX_20== 1) ? 1 : 0"]
pub type DrmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When set to 1, mutes ih_fc_stat2\\[0\\]"]
    #[inline(always)]
    pub fn highpriority_overflow(&self) -> HighpriorityOverflowR {
        HighpriorityOverflowR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set to 1, mutes ih_fc_stat2\\[1\\]"]
    #[inline(always)]
    pub fn lowpriority_overflow(&self) -> LowpriorityOverflowR {
        LowpriorityOverflowR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - When set to 1, mutes ih_fc_stat2\\[4\\]. Reset Value: (HDMI_TX_20== 1) ? 1 : 0"]
    #[inline(always)]
    pub fn drm(&self) -> DrmR {
        DrmR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set to 1, mutes ih_fc_stat2\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn highpriority_overflow(&mut self) -> HighpriorityOverflowW<IhMuteFcStat2Spec> {
        HighpriorityOverflowW::new(self, 0)
    }
    #[doc = "Bit 1 - When set to 1, mutes ih_fc_stat2\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn lowpriority_overflow(&mut self) -> LowpriorityOverflowW<IhMuteFcStat2Spec> {
        LowpriorityOverflowW::new(self, 1)
    }
    #[doc = "Bit 4 - When set to 1, mutes ih_fc_stat2\\[4\\]. Reset Value: (HDMI_TX_20== 1) ? 1 : 0"]
    #[inline(always)]
    #[must_use]
    pub fn drm(&mut self) -> DrmW<IhMuteFcStat2Spec> {
        DrmW::new(self, 4)
    }
}
#[doc = "When set to 1, mutes ih_fc_stat2\\[0\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_mute_fc_stat2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_mute_fc_stat2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IhMuteFcStat2Spec;
impl crate::RegisterSpec for IhMuteFcStat2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ih_mute_fc_stat2::R`](R) reader structure"]
impl crate::Readable for IhMuteFcStat2Spec {}
#[doc = "`write(|w| ..)` method takes [`ih_mute_fc_stat2::W`](W) writer structure"]
impl crate::Writable for IhMuteFcStat2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IH_MUTE_FC_STAT2 to value 0"]
impl crate::Resettable for IhMuteFcStat2Spec {
    const RESET_VALUE: u8 = 0;
}

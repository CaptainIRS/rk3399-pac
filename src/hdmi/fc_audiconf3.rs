#[doc = "Register `FC_AUDICONF3` reader"]
pub type R = crate::R<FcAudiconf3Spec>;
#[doc = "Register `FC_AUDICONF3` writer"]
pub type W = crate::W<FcAudiconf3Spec>;
#[doc = "Field `LSV` reader - Level shift value (for down mixing)"]
pub type LsvR = crate::FieldReader;
#[doc = "Field `LSV` writer - Level shift value (for down mixing)"]
pub type LsvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DM_INH` reader - Down mix enable"]
pub type DmInhR = crate::BitReader;
#[doc = "Field `DM_INH` writer - Down mix enable"]
pub type DmInhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFEPBL` reader - LFE playback information LFEPBL1, LFEPBL0 LFE playback level as compared to the other channels."]
pub type LfepblR = crate::FieldReader;
#[doc = "Field `LFEPBL` writer - LFE playback information LFEPBL1, LFEPBL0 LFE playback level as compared to the other channels."]
pub type LfepblW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Level shift value (for down mixing)"]
    #[inline(always)]
    pub fn lsv(&self) -> LsvR {
        LsvR::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - Down mix enable"]
    #[inline(always)]
    pub fn dm_inh(&self) -> DmInhR {
        DmInhR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - LFE playback information LFEPBL1, LFEPBL0 LFE playback level as compared to the other channels."]
    #[inline(always)]
    pub fn lfepbl(&self) -> LfepblR {
        LfepblR::new((self.bits >> 5) & 3)
    }
}
impl W {
    #[doc = "Bits 0:3 - Level shift value (for down mixing)"]
    #[inline(always)]
    #[must_use]
    pub fn lsv(&mut self) -> LsvW<FcAudiconf3Spec> {
        LsvW::new(self, 0)
    }
    #[doc = "Bit 4 - Down mix enable"]
    #[inline(always)]
    #[must_use]
    pub fn dm_inh(&mut self) -> DmInhW<FcAudiconf3Spec> {
        DmInhW::new(self, 4)
    }
    #[doc = "Bits 5:6 - LFE playback information LFEPBL1, LFEPBL0 LFE playback level as compared to the other channels."]
    #[inline(always)]
    #[must_use]
    pub fn lfepbl(&mut self) -> LfepblW<FcAudiconf3Spec> {
        LfepblW::new(self, 5)
    }
}
#[doc = "Level shift value (for down mixing)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audiconf3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audiconf3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAudiconf3Spec;
impl crate::RegisterSpec for FcAudiconf3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_audiconf3::R`](R) reader structure"]
impl crate::Readable for FcAudiconf3Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_audiconf3::W`](W) writer structure"]
impl crate::Writable for FcAudiconf3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_AUDICONF3 to value 0"]
impl crate::Resettable for FcAudiconf3Spec {
    const RESET_VALUE: u8 = 0;
}

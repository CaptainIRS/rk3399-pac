#[doc = "Register `IH_MUTE_FC_STAT1` reader"]
pub type R = crate::R<IhMuteFcStat1Spec>;
#[doc = "Register `IH_MUTE_FC_STAT1` writer"]
pub type W = crate::W<IhMuteFcStat1Spec>;
#[doc = "Field `AMP` reader - When set to 1, mutes ih_fc_stat1\\[2\\]. Otherwise, this field is a \"spare\" bit with no associated functionality. Reset Value: (HDMI_TX_20== 1) ? 1 : 0"]
pub type AmpR = crate::BitReader;
#[doc = "Field `AMP` writer - When set to 1, mutes ih_fc_stat1\\[2\\]. Otherwise, this field is a \"spare\" bit with no associated functionality. Reset Value: (HDMI_TX_20== 1) ? 1 : 0"]
pub type AmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPD` reader - When set to 1, mutes ih_fc_stat1\\[3\\]"]
pub type SpdR = crate::BitReader;
#[doc = "Field `SPD` writer - When set to 1, mutes ih_fc_stat1\\[3\\]"]
pub type SpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSD` reader - When set to 1, mutes ih_fc_stat1\\[4\\]"]
pub type VsdR = crate::BitReader;
#[doc = "Field `VSD` writer - When set to 1, mutes ih_fc_stat1\\[4\\]"]
pub type VsdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISCR2` reader - When set to 1, mutes ih_fc_stat1\\[5\\]"]
pub type Iscr2R = crate::BitReader;
#[doc = "Field `ISCR2` writer - When set to 1, mutes ih_fc_stat1\\[5\\]"]
pub type Iscr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISCR1` reader - When set to 1, mutes ih_fc_stat1\\[6\\]"]
pub type Iscr1R = crate::BitReader;
#[doc = "Field `ISCR1` writer - When set to 1, mutes ih_fc_stat1\\[6\\]"]
pub type Iscr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GMD` reader - When set to 1, mutes ih_fc_stat1\\[7\\]"]
pub type GmdR = crate::BitReader;
#[doc = "Field `GMD` writer - When set to 1, mutes ih_fc_stat1\\[7\\]"]
pub type GmdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - When set to 1, mutes ih_fc_stat1\\[2\\]. Otherwise, this field is a \"spare\" bit with no associated functionality. Reset Value: (HDMI_TX_20== 1) ? 1 : 0"]
    #[inline(always)]
    pub fn amp(&self) -> AmpR {
        AmpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When set to 1, mutes ih_fc_stat1\\[3\\]"]
    #[inline(always)]
    pub fn spd(&self) -> SpdR {
        SpdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When set to 1, mutes ih_fc_stat1\\[4\\]"]
    #[inline(always)]
    pub fn vsd(&self) -> VsdR {
        VsdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When set to 1, mutes ih_fc_stat1\\[5\\]"]
    #[inline(always)]
    pub fn iscr2(&self) -> Iscr2R {
        Iscr2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When set to 1, mutes ih_fc_stat1\\[6\\]"]
    #[inline(always)]
    pub fn iscr1(&self) -> Iscr1R {
        Iscr1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When set to 1, mutes ih_fc_stat1\\[7\\]"]
    #[inline(always)]
    pub fn gmd(&self) -> GmdR {
        GmdR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - When set to 1, mutes ih_fc_stat1\\[2\\]. Otherwise, this field is a \"spare\" bit with no associated functionality. Reset Value: (HDMI_TX_20== 1) ? 1 : 0"]
    #[inline(always)]
    #[must_use]
    pub fn amp(&mut self) -> AmpW<IhMuteFcStat1Spec> {
        AmpW::new(self, 2)
    }
    #[doc = "Bit 3 - When set to 1, mutes ih_fc_stat1\\[3\\]"]
    #[inline(always)]
    #[must_use]
    pub fn spd(&mut self) -> SpdW<IhMuteFcStat1Spec> {
        SpdW::new(self, 3)
    }
    #[doc = "Bit 4 - When set to 1, mutes ih_fc_stat1\\[4\\]"]
    #[inline(always)]
    #[must_use]
    pub fn vsd(&mut self) -> VsdW<IhMuteFcStat1Spec> {
        VsdW::new(self, 4)
    }
    #[doc = "Bit 5 - When set to 1, mutes ih_fc_stat1\\[5\\]"]
    #[inline(always)]
    #[must_use]
    pub fn iscr2(&mut self) -> Iscr2W<IhMuteFcStat1Spec> {
        Iscr2W::new(self, 5)
    }
    #[doc = "Bit 6 - When set to 1, mutes ih_fc_stat1\\[6\\]"]
    #[inline(always)]
    #[must_use]
    pub fn iscr1(&mut self) -> Iscr1W<IhMuteFcStat1Spec> {
        Iscr1W::new(self, 6)
    }
    #[doc = "Bit 7 - When set to 1, mutes ih_fc_stat1\\[7\\]"]
    #[inline(always)]
    #[must_use]
    pub fn gmd(&mut self) -> GmdW<IhMuteFcStat1Spec> {
        GmdW::new(self, 7)
    }
}
#[doc = "When set to 1, mutes ih_fc_stat1\\[2\\]. Otherwise, this field is a \"spare\" bit with no associated functionality.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_mute_fc_stat1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_mute_fc_stat1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IhMuteFcStat1Spec;
impl crate::RegisterSpec for IhMuteFcStat1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ih_mute_fc_stat1::R`](R) reader structure"]
impl crate::Readable for IhMuteFcStat1Spec {}
#[doc = "`write(|w| ..)` method takes [`ih_mute_fc_stat1::W`](W) writer structure"]
impl crate::Writable for IhMuteFcStat1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IH_MUTE_FC_STAT1 to value 0"]
impl crate::Resettable for IhMuteFcStat1Spec {
    const RESET_VALUE: u8 = 0;
}

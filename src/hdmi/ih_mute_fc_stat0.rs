#[doc = "Register `IH_MUTE_FC_STAT0` reader"]
pub type R = crate::R<IhMuteFcStat0Spec>;
#[doc = "Register `IH_MUTE_FC_STAT0` writer"]
pub type W = crate::W<IhMuteFcStat0Spec>;
#[doc = "Field `NULL` reader - When set to 1, mutes ih_fc_stat0\\[0\\]"]
pub type NullR = crate::BitReader;
#[doc = "Field `NULL` writer - When set to 1, mutes ih_fc_stat0\\[0\\]"]
pub type NullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACR` reader - When set to 1, mutes ih_fc_stat0\\[1\\]"]
pub type AcrR = crate::BitReader;
#[doc = "Field `ACR` writer - When set to 1, mutes ih_fc_stat0\\[1\\]"]
pub type AcrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUDS` reader - When set to 1, mutes ih_fc_stat0\\[2\\]"]
pub type AudsR = crate::BitReader;
#[doc = "Field `AUDS` writer - When set to 1, mutes ih_fc_stat0\\[2\\]"]
pub type AudsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NVBI` reader - When set to 1, mutes ih_fc_stat0\\[3\\]. Otherwise, this field is a \"spare\" bit with no associated functionality. Reset Value: (HDMI_TX_20== 1) ? 1 : 0"]
pub type NvbiR = crate::BitReader;
#[doc = "Field `NVBI` writer - When set to 1, mutes ih_fc_stat0\\[3\\]. Otherwise, this field is a \"spare\" bit with no associated functionality. Reset Value: (HDMI_TX_20== 1) ? 1 : 0"]
pub type NvbiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAS` reader - When set to 1, mutes ih_fc_stat0\\[4\\]. Otherwise, this field is a \"spare\" bit with no associated functionality. Reset Value: (HDMI_TX_20== 1) ? 1 : 0"]
pub type MasR = crate::BitReader;
#[doc = "Field `MAS` writer - When set to 1, mutes ih_fc_stat0\\[4\\]. Otherwise, this field is a \"spare\" bit with no associated functionality. Reset Value: (HDMI_TX_20== 1) ? 1 : 0"]
pub type MasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBR` reader - When set to 1, mutes ih_fc_stat0\\[5\\]"]
pub type HbrR = crate::BitReader;
#[doc = "Field `HBR` writer - When set to 1, mutes ih_fc_stat0\\[5\\]"]
pub type HbrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACP` reader - When set to 1, mutes ih_fc_stat0\\[6\\]"]
pub type AcpR = crate::BitReader;
#[doc = "Field `ACP` writer - When set to 1, mutes ih_fc_stat0\\[6\\]"]
pub type AcpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUDI` reader - When set to 1, mutes ih_fc_stat0\\[7\\]"]
pub type AudiR = crate::BitReader;
#[doc = "Field `AUDI` writer - When set to 1, mutes ih_fc_stat0\\[7\\]"]
pub type AudiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When set to 1, mutes ih_fc_stat0\\[0\\]"]
    #[inline(always)]
    pub fn null(&self) -> NullR {
        NullR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set to 1, mutes ih_fc_stat0\\[1\\]"]
    #[inline(always)]
    pub fn acr(&self) -> AcrR {
        AcrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When set to 1, mutes ih_fc_stat0\\[2\\]"]
    #[inline(always)]
    pub fn auds(&self) -> AudsR {
        AudsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When set to 1, mutes ih_fc_stat0\\[3\\]. Otherwise, this field is a \"spare\" bit with no associated functionality. Reset Value: (HDMI_TX_20== 1) ? 1 : 0"]
    #[inline(always)]
    pub fn nvbi(&self) -> NvbiR {
        NvbiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When set to 1, mutes ih_fc_stat0\\[4\\]. Otherwise, this field is a \"spare\" bit with no associated functionality. Reset Value: (HDMI_TX_20== 1) ? 1 : 0"]
    #[inline(always)]
    pub fn mas(&self) -> MasR {
        MasR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When set to 1, mutes ih_fc_stat0\\[5\\]"]
    #[inline(always)]
    pub fn hbr(&self) -> HbrR {
        HbrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When set to 1, mutes ih_fc_stat0\\[6\\]"]
    #[inline(always)]
    pub fn acp(&self) -> AcpR {
        AcpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When set to 1, mutes ih_fc_stat0\\[7\\]"]
    #[inline(always)]
    pub fn audi(&self) -> AudiR {
        AudiR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set to 1, mutes ih_fc_stat0\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn null(&mut self) -> NullW<IhMuteFcStat0Spec> {
        NullW::new(self, 0)
    }
    #[doc = "Bit 1 - When set to 1, mutes ih_fc_stat0\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn acr(&mut self) -> AcrW<IhMuteFcStat0Spec> {
        AcrW::new(self, 1)
    }
    #[doc = "Bit 2 - When set to 1, mutes ih_fc_stat0\\[2\\]"]
    #[inline(always)]
    #[must_use]
    pub fn auds(&mut self) -> AudsW<IhMuteFcStat0Spec> {
        AudsW::new(self, 2)
    }
    #[doc = "Bit 3 - When set to 1, mutes ih_fc_stat0\\[3\\]. Otherwise, this field is a \"spare\" bit with no associated functionality. Reset Value: (HDMI_TX_20== 1) ? 1 : 0"]
    #[inline(always)]
    #[must_use]
    pub fn nvbi(&mut self) -> NvbiW<IhMuteFcStat0Spec> {
        NvbiW::new(self, 3)
    }
    #[doc = "Bit 4 - When set to 1, mutes ih_fc_stat0\\[4\\]. Otherwise, this field is a \"spare\" bit with no associated functionality. Reset Value: (HDMI_TX_20== 1) ? 1 : 0"]
    #[inline(always)]
    #[must_use]
    pub fn mas(&mut self) -> MasW<IhMuteFcStat0Spec> {
        MasW::new(self, 4)
    }
    #[doc = "Bit 5 - When set to 1, mutes ih_fc_stat0\\[5\\]"]
    #[inline(always)]
    #[must_use]
    pub fn hbr(&mut self) -> HbrW<IhMuteFcStat0Spec> {
        HbrW::new(self, 5)
    }
    #[doc = "Bit 6 - When set to 1, mutes ih_fc_stat0\\[6\\]"]
    #[inline(always)]
    #[must_use]
    pub fn acp(&mut self) -> AcpW<IhMuteFcStat0Spec> {
        AcpW::new(self, 6)
    }
    #[doc = "Bit 7 - When set to 1, mutes ih_fc_stat0\\[7\\]"]
    #[inline(always)]
    #[must_use]
    pub fn audi(&mut self) -> AudiW<IhMuteFcStat0Spec> {
        AudiW::new(self, 7)
    }
}
#[doc = "When set to 1, mutes ih_fc_stat0\\[0\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_mute_fc_stat0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_mute_fc_stat0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IhMuteFcStat0Spec;
impl crate::RegisterSpec for IhMuteFcStat0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ih_mute_fc_stat0::R`](R) reader structure"]
impl crate::Readable for IhMuteFcStat0Spec {}
#[doc = "`write(|w| ..)` method takes [`ih_mute_fc_stat0::W`](W) writer structure"]
impl crate::Writable for IhMuteFcStat0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IH_MUTE_FC_STAT0 to value 0"]
impl crate::Resettable for IhMuteFcStat0Spec {
    const RESET_VALUE: u8 = 0;
}

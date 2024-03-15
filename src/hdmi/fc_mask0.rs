#[doc = "Register `FC_MASK0` reader"]
pub type R = crate::R<FcMask0Spec>;
#[doc = "Register `FC_MASK0` writer"]
pub type W = crate::W<FcMask0Spec>;
#[doc = "Field `NULL` reader - Mask bit for FC_INT0.NULL interrupt bit"]
pub type NullR = crate::BitReader;
#[doc = "Field `NULL` writer - Mask bit for FC_INT0.NULL interrupt bit"]
pub type NullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACR` reader - Mask bit for FC_INT0.ACR interrupt bit"]
pub type AcrR = crate::BitReader;
#[doc = "Field `ACR` writer - Mask bit for FC_INT0.ACR interrupt bit"]
pub type AcrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUDS` reader - Mask bit for FC_INT0.AUDS interrupt bit"]
pub type AudsR = crate::BitReader;
#[doc = "Field `AUDS` writer - Mask bit for FC_INT0.AUDS interrupt bit"]
pub type AudsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NVBI` reader - Mask bit for FC_INT0.NVBI interrupt bit. Otherwise, this field is a \"spare\" bit with no associated functionality. Reset Value: (HDMI_TX_20== 1) ? 1 : 0"]
pub type NvbiR = crate::BitReader;
#[doc = "Field `NVBI` writer - Mask bit for FC_INT0.NVBI interrupt bit. Otherwise, this field is a \"spare\" bit with no associated functionality. Reset Value: (HDMI_TX_20== 1) ? 1 : 0"]
pub type NvbiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAS` reader - Mask bit for FC_INT0.MAS interrupt bit. Otherwise, this field is a \"spare\" bit with no associated functionality. Reset Value: (HDMI_TX_20== 1) ? 1 : 0"]
pub type MasR = crate::BitReader;
#[doc = "Field `MAS` writer - Mask bit for FC_INT0.MAS interrupt bit. Otherwise, this field is a \"spare\" bit with no associated functionality. Reset Value: (HDMI_TX_20== 1) ? 1 : 0"]
pub type MasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBR` reader - Mask bit for FC_INT0.HBR interrupt bit"]
pub type HbrR = crate::BitReader;
#[doc = "Field `HBR` writer - Mask bit for FC_INT0.HBR interrupt bit"]
pub type HbrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACP` reader - Mask bit for FC_INT0.ACP interrupt bit"]
pub type AcpR = crate::BitReader;
#[doc = "Field `ACP` writer - Mask bit for FC_INT0.ACP interrupt bit"]
pub type AcpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUDI` reader - Mask bit for FC_INT0.AUDI interrupt bit"]
pub type AudiR = crate::BitReader;
#[doc = "Field `AUDI` writer - Mask bit for FC_INT0.AUDI interrupt bit"]
pub type AudiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mask bit for FC_INT0.NULL interrupt bit"]
    #[inline(always)]
    pub fn null(&self) -> NullR {
        NullR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask bit for FC_INT0.ACR interrupt bit"]
    #[inline(always)]
    pub fn acr(&self) -> AcrR {
        AcrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask bit for FC_INT0.AUDS interrupt bit"]
    #[inline(always)]
    pub fn auds(&self) -> AudsR {
        AudsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask bit for FC_INT0.NVBI interrupt bit. Otherwise, this field is a \"spare\" bit with no associated functionality. Reset Value: (HDMI_TX_20== 1) ? 1 : 0"]
    #[inline(always)]
    pub fn nvbi(&self) -> NvbiR {
        NvbiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask bit for FC_INT0.MAS interrupt bit. Otherwise, this field is a \"spare\" bit with no associated functionality. Reset Value: (HDMI_TX_20== 1) ? 1 : 0"]
    #[inline(always)]
    pub fn mas(&self) -> MasR {
        MasR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask bit for FC_INT0.HBR interrupt bit"]
    #[inline(always)]
    pub fn hbr(&self) -> HbrR {
        HbrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mask bit for FC_INT0.ACP interrupt bit"]
    #[inline(always)]
    pub fn acp(&self) -> AcpR {
        AcpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mask bit for FC_INT0.AUDI interrupt bit"]
    #[inline(always)]
    pub fn audi(&self) -> AudiR {
        AudiR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for FC_INT0.NULL interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn null(&mut self) -> NullW<FcMask0Spec> {
        NullW::new(self, 0)
    }
    #[doc = "Bit 1 - Mask bit for FC_INT0.ACR interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn acr(&mut self) -> AcrW<FcMask0Spec> {
        AcrW::new(self, 1)
    }
    #[doc = "Bit 2 - Mask bit for FC_INT0.AUDS interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn auds(&mut self) -> AudsW<FcMask0Spec> {
        AudsW::new(self, 2)
    }
    #[doc = "Bit 3 - Mask bit for FC_INT0.NVBI interrupt bit. Otherwise, this field is a \"spare\" bit with no associated functionality. Reset Value: (HDMI_TX_20== 1) ? 1 : 0"]
    #[inline(always)]
    #[must_use]
    pub fn nvbi(&mut self) -> NvbiW<FcMask0Spec> {
        NvbiW::new(self, 3)
    }
    #[doc = "Bit 4 - Mask bit for FC_INT0.MAS interrupt bit. Otherwise, this field is a \"spare\" bit with no associated functionality. Reset Value: (HDMI_TX_20== 1) ? 1 : 0"]
    #[inline(always)]
    #[must_use]
    pub fn mas(&mut self) -> MasW<FcMask0Spec> {
        MasW::new(self, 4)
    }
    #[doc = "Bit 5 - Mask bit for FC_INT0.HBR interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn hbr(&mut self) -> HbrW<FcMask0Spec> {
        HbrW::new(self, 5)
    }
    #[doc = "Bit 6 - Mask bit for FC_INT0.ACP interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn acp(&mut self) -> AcpW<FcMask0Spec> {
        AcpW::new(self, 6)
    }
    #[doc = "Bit 7 - Mask bit for FC_INT0.AUDI interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn audi(&mut self) -> AudiW<FcMask0Spec> {
        AudiW::new(self, 7)
    }
}
#[doc = "Mask bit for FC_INT0.NULL interrupt bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_mask0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_mask0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcMask0Spec;
impl crate::RegisterSpec for FcMask0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_mask0::R`](R) reader structure"]
impl crate::Readable for FcMask0Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_mask0::W`](W) writer structure"]
impl crate::Writable for FcMask0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_MASK0 to value 0x25"]
impl crate::Resettable for FcMask0Spec {
    const RESET_VALUE: u8 = 0x25;
}

#[doc = "Register `FC_MASK1` reader"]
pub type R = crate::R<FcMask1Spec>;
#[doc = "Register `FC_MASK1` writer"]
pub type W = crate::W<FcMask1Spec>;
#[doc = "Field `GCP` reader - Mask bit for FC_INT1.GCP interrupt bit"]
pub type GcpR = crate::BitReader;
#[doc = "Field `GCP` writer - Mask bit for FC_INT1.GCP interrupt bit"]
pub type GcpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVI` reader - Mask bit for FC_INT1.AVI interrupt bit"]
pub type AviR = crate::BitReader;
#[doc = "Field `AVI` writer - Mask bit for FC_INT1.AVI interrupt bit"]
pub type AviW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMP` reader - Mask bit for FC_INT1.AMP interrupt bit. Otherwise,\n\nthis field is a \"spare\" bit with no associated\n\nfunctionality.\n\nReset Value: (HDMI_TX_20== 1) ? 1 : 0"]
pub type AmpR = crate::BitReader;
#[doc = "Field `AMP` writer - Mask bit for FC_INT1.AMP interrupt bit. Otherwise,\n\nthis field is a \"spare\" bit with no associated\n\nfunctionality.\n\nReset Value: (HDMI_TX_20== 1) ? 1 : 0"]
pub type AmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPD` reader - Mask bit for FC_INT1.SPD interrupt bit"]
pub type SpdR = crate::BitReader;
#[doc = "Field `SPD` writer - Mask bit for FC_INT1.SPD interrupt bit"]
pub type SpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSD` reader - Mask bit for FC_INT1.VSD interrupt bit"]
pub type VsdR = crate::BitReader;
#[doc = "Field `VSD` writer - Mask bit for FC_INT1.VSD interrupt bit"]
pub type VsdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISCR2` reader - Mask bit for FC_INT1.ISRC2 interrupt bit"]
pub type Iscr2R = crate::BitReader;
#[doc = "Field `ISCR2` writer - Mask bit for FC_INT1.ISRC2 interrupt bit"]
pub type Iscr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISCR1` reader - Mask bit for FC_INT1.ISRC1 interrupt bit"]
pub type Iscr1R = crate::BitReader;
#[doc = "Field `ISCR1` writer - Mask bit for FC_INT1.ISRC1 interrupt bit"]
pub type Iscr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GMD` reader - Mask bit for FC_INT1.GMD interrupt bit"]
pub type GmdR = crate::BitReader;
#[doc = "Field `GMD` writer - Mask bit for FC_INT1.GMD interrupt bit"]
pub type GmdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mask bit for FC_INT1.GCP interrupt bit"]
    #[inline(always)]
    pub fn gcp(&self) -> GcpR {
        GcpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask bit for FC_INT1.AVI interrupt bit"]
    #[inline(always)]
    pub fn avi(&self) -> AviR {
        AviR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask bit for FC_INT1.AMP interrupt bit. Otherwise,\n\nthis field is a \"spare\" bit with no associated\n\nfunctionality.\n\nReset Value: (HDMI_TX_20== 1) ? 1 : 0"]
    #[inline(always)]
    pub fn amp(&self) -> AmpR {
        AmpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask bit for FC_INT1.SPD interrupt bit"]
    #[inline(always)]
    pub fn spd(&self) -> SpdR {
        SpdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask bit for FC_INT1.VSD interrupt bit"]
    #[inline(always)]
    pub fn vsd(&self) -> VsdR {
        VsdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask bit for FC_INT1.ISRC2 interrupt bit"]
    #[inline(always)]
    pub fn iscr2(&self) -> Iscr2R {
        Iscr2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mask bit for FC_INT1.ISRC1 interrupt bit"]
    #[inline(always)]
    pub fn iscr1(&self) -> Iscr1R {
        Iscr1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mask bit for FC_INT1.GMD interrupt bit"]
    #[inline(always)]
    pub fn gmd(&self) -> GmdR {
        GmdR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for FC_INT1.GCP interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn gcp(&mut self) -> GcpW<FcMask1Spec> {
        GcpW::new(self, 0)
    }
    #[doc = "Bit 1 - Mask bit for FC_INT1.AVI interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn avi(&mut self) -> AviW<FcMask1Spec> {
        AviW::new(self, 1)
    }
    #[doc = "Bit 2 - Mask bit for FC_INT1.AMP interrupt bit. Otherwise,\n\nthis field is a \"spare\" bit with no associated\n\nfunctionality.\n\nReset Value: (HDMI_TX_20== 1) ? 1 : 0"]
    #[inline(always)]
    #[must_use]
    pub fn amp(&mut self) -> AmpW<FcMask1Spec> {
        AmpW::new(self, 2)
    }
    #[doc = "Bit 3 - Mask bit for FC_INT1.SPD interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn spd(&mut self) -> SpdW<FcMask1Spec> {
        SpdW::new(self, 3)
    }
    #[doc = "Bit 4 - Mask bit for FC_INT1.VSD interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn vsd(&mut self) -> VsdW<FcMask1Spec> {
        VsdW::new(self, 4)
    }
    #[doc = "Bit 5 - Mask bit for FC_INT1.ISRC2 interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn iscr2(&mut self) -> Iscr2W<FcMask1Spec> {
        Iscr2W::new(self, 5)
    }
    #[doc = "Bit 6 - Mask bit for FC_INT1.ISRC1 interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn iscr1(&mut self) -> Iscr1W<FcMask1Spec> {
        Iscr1W::new(self, 6)
    }
    #[doc = "Bit 7 - Mask bit for FC_INT1.GMD interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn gmd(&mut self) -> GmdW<FcMask1Spec> {
        GmdW::new(self, 7)
    }
}
#[doc = "Frame Composer Packet Interrupt Mask Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_mask1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_mask1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcMask1Spec;
impl crate::RegisterSpec for FcMask1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_mask1::R`](R) reader structure"]
impl crate::Readable for FcMask1Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_mask1::W`](W) writer structure"]
impl crate::Writable for FcMask1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_MASK1 to value 0"]
impl crate::Resettable for FcMask1Spec {
    const RESET_VALUE: u8 = 0;
}

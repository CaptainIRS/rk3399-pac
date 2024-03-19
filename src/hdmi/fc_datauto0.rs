#[doc = "Register `FC_DATAUTO0` reader"]
pub type R = crate::R<FcDatauto0Spec>;
#[doc = "Register `FC_DATAUTO0` writer"]
pub type W = crate::W<FcDatauto0Spec>;
#[doc = "Field `ACP_AUTO` reader - Enables ACP automatic packet scheduling"]
pub type AcpAutoR = crate::BitReader;
#[doc = "Field `ACP_AUTO` writer - Enables ACP automatic packet scheduling"]
pub type AcpAutoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISCR1_AUTO` reader - Enables ISRC1 automatic packet scheduling"]
pub type Iscr1AutoR = crate::BitReader;
#[doc = "Field `ISCR1_AUTO` writer - Enables ISRC1 automatic packet scheduling"]
pub type Iscr1AutoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISCR2_AUTO` reader - Enables ISRC2 automatic packet scheduling"]
pub type Iscr2AutoR = crate::BitReader;
#[doc = "Field `ISCR2_AUTO` writer - Enables ISRC2 automatic packet scheduling"]
pub type Iscr2AutoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSD_AUTO` reader - Enables VSD automatic packet scheduling"]
pub type VsdAutoR = crate::BitReader;
#[doc = "Field `VSD_AUTO` writer - Enables VSD automatic packet scheduling"]
pub type VsdAutoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPD_AUTO` reader - Enables SPD automatic packet scheduling"]
pub type SpdAutoR = crate::BitReader;
#[doc = "Field `SPD_AUTO` writer - Enables SPD automatic packet scheduling"]
pub type SpdAutoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables ACP automatic packet scheduling"]
    #[inline(always)]
    pub fn acp_auto(&self) -> AcpAutoR {
        AcpAutoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables ISRC1 automatic packet scheduling"]
    #[inline(always)]
    pub fn iscr1_auto(&self) -> Iscr1AutoR {
        Iscr1AutoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables ISRC2 automatic packet scheduling"]
    #[inline(always)]
    pub fn iscr2_auto(&self) -> Iscr2AutoR {
        Iscr2AutoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables VSD automatic packet scheduling"]
    #[inline(always)]
    pub fn vsd_auto(&self) -> VsdAutoR {
        VsdAutoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables SPD automatic packet scheduling"]
    #[inline(always)]
    pub fn spd_auto(&self) -> SpdAutoR {
        SpdAutoR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables ACP automatic packet scheduling"]
    #[inline(always)]
    #[must_use]
    pub fn acp_auto(&mut self) -> AcpAutoW<FcDatauto0Spec> {
        AcpAutoW::new(self, 0)
    }
    #[doc = "Bit 1 - Enables ISRC1 automatic packet scheduling"]
    #[inline(always)]
    #[must_use]
    pub fn iscr1_auto(&mut self) -> Iscr1AutoW<FcDatauto0Spec> {
        Iscr1AutoW::new(self, 1)
    }
    #[doc = "Bit 2 - Enables ISRC2 automatic packet scheduling"]
    #[inline(always)]
    #[must_use]
    pub fn iscr2_auto(&mut self) -> Iscr2AutoW<FcDatauto0Spec> {
        Iscr2AutoW::new(self, 2)
    }
    #[doc = "Bit 3 - Enables VSD automatic packet scheduling"]
    #[inline(always)]
    #[must_use]
    pub fn vsd_auto(&mut self) -> VsdAutoW<FcDatauto0Spec> {
        VsdAutoW::new(self, 3)
    }
    #[doc = "Bit 4 - Enables SPD automatic packet scheduling"]
    #[inline(always)]
    #[must_use]
    pub fn spd_auto(&mut self) -> SpdAutoW<FcDatauto0Spec> {
        SpdAutoW::new(self, 4)
    }
}
#[doc = "Frame Composer Data Island Auto Packet Scheduling Register 0\n\nConfigures the Frame Composer RDRB(1)/Manual(0) data island packet insertion for SPD,\n\nVSD, ISRC2, ISRC1 and ACP packets. On RDRB mode the described packet scheduling is\n\ncontrolled by registers FC_DATAUTO1 and FC_DATAUTO2, while in Manual mode register\n\nFC_DATMAN requests to FC the insertion of the requested packet.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_datauto0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_datauto0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcDatauto0Spec;
impl crate::RegisterSpec for FcDatauto0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_datauto0::R`](R) reader structure"]
impl crate::Readable for FcDatauto0Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_datauto0::W`](W) writer structure"]
impl crate::Writable for FcDatauto0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_DATAUTO0 to value 0"]
impl crate::Resettable for FcDatauto0Spec {
    const RESET_VALUE: u8 = 0;
}

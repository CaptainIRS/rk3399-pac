#[doc = "Register `FC_GCP` reader"]
pub type R = crate::R<FcGcpSpec>;
#[doc = "Register `FC_GCP` writer"]
pub type W = crate::W<FcGcpSpec>;
#[doc = "Field `CLEAR_AVMUTE` reader - Value of \"clear_avmute\" in the GCP packet"]
pub type ClearAvmuteR = crate::BitReader;
#[doc = "Field `CLEAR_AVMUTE` writer - Value of \"clear_avmute\" in the GCP packet"]
pub type ClearAvmuteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_AVMUTE` reader - Value of \"set_avmute\" in the GCP packet Once the AVmute is set, the frame composer schedules the GCP packet with AVmute set in the packet scheduler to be sent once (may only be transmitted between the active edge of VSYNC and 384 pixels following this edge)."]
pub type SetAvmuteR = crate::BitReader;
#[doc = "Field `SET_AVMUTE` writer - Value of \"set_avmute\" in the GCP packet Once the AVmute is set, the frame composer schedules the GCP packet with AVmute set in the packet scheduler to be sent once (may only be transmitted between the active edge of VSYNC and 384 pixels following this edge)."]
pub type SetAvmuteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEFAULT_PHASE` reader - Value of \"default_phase\" in the GCP packet. This data must be equal to the default phase used at Video Packetizer packing machine."]
pub type DefaultPhaseR = crate::BitReader;
#[doc = "Field `DEFAULT_PHASE` writer - Value of \"default_phase\" in the GCP packet. This data must be equal to the default phase used at Video Packetizer packing machine."]
pub type DefaultPhaseW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Value of \"clear_avmute\" in the GCP packet"]
    #[inline(always)]
    pub fn clear_avmute(&self) -> ClearAvmuteR {
        ClearAvmuteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Value of \"set_avmute\" in the GCP packet Once the AVmute is set, the frame composer schedules the GCP packet with AVmute set in the packet scheduler to be sent once (may only be transmitted between the active edge of VSYNC and 384 pixels following this edge)."]
    #[inline(always)]
    pub fn set_avmute(&self) -> SetAvmuteR {
        SetAvmuteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Value of \"default_phase\" in the GCP packet. This data must be equal to the default phase used at Video Packetizer packing machine."]
    #[inline(always)]
    pub fn default_phase(&self) -> DefaultPhaseR {
        DefaultPhaseR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Value of \"clear_avmute\" in the GCP packet"]
    #[inline(always)]
    #[must_use]
    pub fn clear_avmute(&mut self) -> ClearAvmuteW<FcGcpSpec> {
        ClearAvmuteW::new(self, 0)
    }
    #[doc = "Bit 1 - Value of \"set_avmute\" in the GCP packet Once the AVmute is set, the frame composer schedules the GCP packet with AVmute set in the packet scheduler to be sent once (may only be transmitted between the active edge of VSYNC and 384 pixels following this edge)."]
    #[inline(always)]
    #[must_use]
    pub fn set_avmute(&mut self) -> SetAvmuteW<FcGcpSpec> {
        SetAvmuteW::new(self, 1)
    }
    #[doc = "Bit 2 - Value of \"default_phase\" in the GCP packet. This data must be equal to the default phase used at Video Packetizer packing machine."]
    #[inline(always)]
    #[must_use]
    pub fn default_phase(&mut self) -> DefaultPhaseW<FcGcpSpec> {
        DefaultPhaseW::new(self, 2)
    }
}
#[doc = "Value of \"clear_avmute\" in the GCP packet\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_gcp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_gcp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcGcpSpec;
impl crate::RegisterSpec for FcGcpSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_gcp::R`](R) reader structure"]
impl crate::Readable for FcGcpSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_gcp::W`](W) writer structure"]
impl crate::Writable for FcGcpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_GCP to value 0"]
impl crate::Resettable for FcGcpSpec {
    const RESET_VALUE: u8 = 0;
}

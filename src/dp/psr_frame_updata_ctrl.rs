#[doc = "Register `PSR_FRAME_UPDATA_CTRL` reader"]
pub type R = crate::R<PsrFrameUpdataCtrlSpec>;
#[doc = "Register `PSR_FRAME_UPDATA_CTRL` writer"]
pub type W = crate::W<PsrFrameUpdataCtrlSpec>;
#[doc = "Field `PSR_VSC_PACKET_VERSION` reader - PSR VSC packet version select. 1 = PSR 2. 0 = PSR 1."]
pub type PsrVscPacketVersionR = crate::BitReader;
#[doc = "Field `PSR_VSC_PACKET_VERSION` writer - PSR VSC packet version select. 1 = PSR 2. 0 = PSR 1."]
pub type PsrVscPacketVersionW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PSR_FRAME_UP_TYPE` reader - Select PSR Frame Update type. 1 = Burst single frame update. 0 = Single frame update. VSC packet is only sent once after PSR_FRAME_UPDATE is written with 1. IF_EN bit will be self-cleared after the VSYNC leading edge."]
pub type PsrFrameUpTypeR = crate::BitReader;
#[doc = "Field `PSR_FRAME_UP_TYPE` writer - Select PSR Frame Update type. 1 = Burst single frame update. 0 = Single frame update. VSC packet is only sent once after PSR_FRAME_UPDATE is written with 1. IF_EN bit will be self-cleared after the VSYNC leading edge."]
pub type PsrFrameUpTypeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PSR VSC packet version select. 1 = PSR 2. 0 = PSR 1."]
    #[inline(always)]
    pub fn psr_vsc_packet_version(&self) -> PsrVscPacketVersionR {
        PsrVscPacketVersionR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select PSR Frame Update type. 1 = Burst single frame update. 0 = Single frame update. VSC packet is only sent once after PSR_FRAME_UPDATE is written with 1. IF_EN bit will be self-cleared after the VSYNC leading edge."]
    #[inline(always)]
    pub fn psr_frame_up_type(&self) -> PsrFrameUpTypeR {
        PsrFrameUpTypeR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PSR VSC packet version select. 1 = PSR 2. 0 = PSR 1."]
    #[inline(always)]
    #[must_use]
    pub fn psr_vsc_packet_version(&mut self) -> PsrVscPacketVersionW<PsrFrameUpdataCtrlSpec> {
        PsrVscPacketVersionW::new(self, 0)
    }
    #[doc = "Bit 1 - Select PSR Frame Update type. 1 = Burst single frame update. 0 = Single frame update. VSC packet is only sent once after PSR_FRAME_UPDATE is written with 1. IF_EN bit will be self-cleared after the VSYNC leading edge."]
    #[inline(always)]
    #[must_use]
    pub fn psr_frame_up_type(&mut self) -> PsrFrameUpTypeW<PsrFrameUpdataCtrlSpec> {
        PsrFrameUpTypeW::new(self, 1)
    }
}
#[doc = "Frame update control for PSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr_frame_updata_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr_frame_updata_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsrFrameUpdataCtrlSpec;
impl crate::RegisterSpec for PsrFrameUpdataCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psr_frame_updata_ctrl::R`](R) reader structure"]
impl crate::Readable for PsrFrameUpdataCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`psr_frame_updata_ctrl::W`](W) writer structure"]
impl crate::Writable for PsrFrameUpdataCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets PSR_FRAME_UPDATA_CTRL to value 0"]
impl crate::Resettable for PsrFrameUpdataCtrlSpec {
    const RESET_VALUE: u32 = 0;
}

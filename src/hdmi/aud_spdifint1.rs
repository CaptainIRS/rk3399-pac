#[doc = "Register `AUD_SPDIFINT1` reader"]
pub type R = crate::R<AudSpdifint1Spec>;
#[doc = "Register `AUD_SPDIFINT1` writer"]
pub type W = crate::W<AudSpdifint1Spec>;
#[doc = "Field `FIFO_OVERRUN_MASK` reader - FIFO overrun mask"]
pub type FifoOverrunMaskR = crate::BitReader;
#[doc = "Field `FIFO_OVERRUN_MASK` writer - FIFO overrun mask"]
pub type FifoOverrunMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - FIFO overrun mask"]
    #[inline(always)]
    pub fn fifo_overrun_mask(&self) -> FifoOverrunMaskR {
        FifoOverrunMaskR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - FIFO overrun mask"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_overrun_mask(&mut self) -> FifoOverrunMaskW<AudSpdifint1Spec> {
        FifoOverrunMaskW::new(self, 4)
    }
}
#[doc = "Reserved for future use.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_spdifint1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_spdifint1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudSpdifint1Spec;
impl crate::RegisterSpec for AudSpdifint1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`aud_spdifint1::R`](R) reader structure"]
impl crate::Readable for AudSpdifint1Spec {}
#[doc = "`write(|w| ..)` method takes [`aud_spdifint1::W`](W) writer structure"]
impl crate::Writable for AudSpdifint1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AUD_SPDIFINT1 to value 0x10"]
impl crate::Resettable for AudSpdifint1Spec {
    const RESET_VALUE: u8 = 0x10;
}

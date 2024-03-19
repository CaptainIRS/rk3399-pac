#[doc = "Register `MC_SWRSTZREQ_2` reader"]
pub type R = crate::R<McSwrstzreq2Spec>;
#[doc = "Register `MC_SWRSTZREQ_2` writer"]
pub type W = crate::W<McSwrstzreq2Spec>;
#[doc = "Field `AHBDMASWRST_REQ` reader - AHB audio DMA software reset request.\n\nWriting 1'b1 does not result in any action.\n\nWriting 1'b0 to this register resets all AHB audio logic."]
pub type AhbdmaswrstReqR = crate::BitReader;
#[doc = "Field `AHBDMASWRST_REQ` writer - AHB audio DMA software reset request.\n\nWriting 1'b1 does not result in any action.\n\nWriting 1'b0 to this register resets all AHB audio logic."]
pub type AhbdmaswrstReqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AHB audio DMA software reset request.\n\nWriting 1'b1 does not result in any action.\n\nWriting 1'b0 to this register resets all AHB audio logic."]
    #[inline(always)]
    pub fn ahbdmaswrst_req(&self) -> AhbdmaswrstReqR {
        AhbdmaswrstReqR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AHB audio DMA software reset request.\n\nWriting 1'b1 does not result in any action.\n\nWriting 1'b0 to this register resets all AHB audio logic."]
    #[inline(always)]
    #[must_use]
    pub fn ahbdmaswrst_req(&mut self) -> AhbdmaswrstReqW<McSwrstzreq2Spec> {
        AhbdmaswrstReqW::new(self, 0)
    }
}
#[doc = "Main Controller Software Reset Register 2\n\nMain controller software reset request per clock domain. Writing zero to a bit of this\n\nregister results in a signal toggle that indicates a software reset request. This toggle is used\n\nto generate a synchronized reset to the corresponding domain, with one or more clock\n\ncycles.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mc_swrstzreq_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mc_swrstzreq_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McSwrstzreq2Spec;
impl crate::RegisterSpec for McSwrstzreq2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mc_swrstzreq_2::R`](R) reader structure"]
impl crate::Readable for McSwrstzreq2Spec {}
#[doc = "`write(|w| ..)` method takes [`mc_swrstzreq_2::W`](W) writer structure"]
impl crate::Writable for McSwrstzreq2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MC_SWRSTZREQ_2 to value 0"]
impl crate::Resettable for McSwrstzreq2Spec {
    const RESET_VALUE: u8 = 0;
}

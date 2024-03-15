#[doc = "Register `PCIE_DMA_CHANNEL_1_CONTROL` reader"]
pub type R = crate::R<PcieDmaChannel1ControlSpec>;
#[doc = "Register `PCIE_DMA_CHANNEL_1_CONTROL` writer"]
pub type W = crate::W<PcieDmaChannel1ControlSpec>;
#[doc = "Field `go` reader - Go command bit \\[go\\]
Kicks off the uDMA channel controller to fetch valid Outbound or Inbound linked list"]
pub type GoR = crate::BitReader;
#[doc = "Field `go` writer - Go command bit \\[go\\]
Kicks off the uDMA channel controller to fetch valid Outbound or Inbound linked list"]
pub type GoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ob_not_ib` reader - Inbound or outbound select \\[ob_not_ib\\]
Determines the direction of the DMA transfer"]
pub type ObNotIbR = crate::BitReader;
#[doc = "Field `ob_not_ib` writer - Inbound or outbound select \\[ob_not_ib\\]
Determines the direction of the DMA transfer"]
pub type ObNotIbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Go command bit \\[go\\]
Kicks off the uDMA channel controller to fetch valid Outbound or Inbound linked list"]
    #[inline(always)]
    pub fn go(&self) -> GoR {
        GoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Inbound or outbound select \\[ob_not_ib\\]
Determines the direction of the DMA transfer"]
    #[inline(always)]
    pub fn ob_not_ib(&self) -> ObNotIbR {
        ObNotIbR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Go command bit \\[go\\]
Kicks off the uDMA channel controller to fetch valid Outbound or Inbound linked list"]
    #[inline(always)]
    #[must_use]
    pub fn go(&mut self) -> GoW<PcieDmaChannel1ControlSpec> {
        GoW::new(self, 0)
    }
    #[doc = "Bit 1 - Inbound or outbound select \\[ob_not_ib\\]
Determines the direction of the DMA transfer"]
    #[inline(always)]
    #[must_use]
    pub fn ob_not_ib(&mut self) -> ObNotIbW<PcieDmaChannel1ControlSpec> {
        ObNotIbW::new(self, 1)
    }
}
#[doc = "PCIe DMA Channel 1 Control Register Reserved for future use\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_channel_1_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_channel_1_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieDmaChannel1ControlSpec;
impl crate::RegisterSpec for PcieDmaChannel1ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_dma_channel_1_control::R`](R) reader structure"]
impl crate::Readable for PcieDmaChannel1ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_dma_channel_1_control::W`](W) writer structure"]
impl crate::Writable for PcieDmaChannel1ControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_DMA_CHANNEL_1_CONTROL to value 0"]
impl crate::Resettable for PcieDmaChannel1ControlSpec {
    const RESET_VALUE: u32 = 0;
}

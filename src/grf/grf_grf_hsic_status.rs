#[doc = "Register `GRF_GRF_HSIC_STATUS` reader"]
pub type R = crate::R<GrfGrfHsicStatusSpec>;
#[doc = "Register `GRF_GRF_HSIC_STATUS` writer"]
pub type W = crate::W<GrfGrfHsicStatusSpec>;
#[doc = "Field `HSIC_EHCI_XFER_CNT` reader - hsic_ehci_xfer_cnt"]
pub type HsicEhciXferCntR = crate::FieldReader<u16>;
#[doc = "Field `HSIC_EHCI_XFER_CNT` writer - hsic_ehci_xfer_cnt"]
pub type HsicEhciXferCntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `HSIC_EHCI_USBSTS` reader - hsic_ehci_usbsts"]
pub type HsicEhciUsbstsR = crate::FieldReader;
#[doc = "Field `HSIC_EHCI_USBSTS` writer - hsic_ehci_usbsts"]
pub type HsicEhciUsbstsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `HSIC_EHCI_LPSMS_STATE` reader - hsic_ehci_lpsms_state"]
pub type HsicEhciLpsmsStateR = crate::FieldReader;
#[doc = "Field `HSIC_EHCI_LPSMS_STATE` writer - hsic_ehci_lpsms_state"]
pub type HsicEhciLpsmsStateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HSIC_EHCI_XFER_PRDC` reader - hsic_ehci_xfer_prdc"]
pub type HsicEhciXferPrdcR = crate::BitReader;
#[doc = "Field `HSIC_EHCI_XFER_PRDC` writer - hsic_ehci_xfer_prdc"]
pub type HsicEhciXferPrdcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - hsic_ehci_xfer_cnt"]
    #[inline(always)]
    pub fn hsic_ehci_xfer_cnt(&self) -> HsicEhciXferCntR {
        HsicEhciXferCntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:16 - hsic_ehci_usbsts"]
    #[inline(always)]
    pub fn hsic_ehci_usbsts(&self) -> HsicEhciUsbstsR {
        HsicEhciUsbstsR::new(((self.bits >> 11) & 0x3f) as u8)
    }
    #[doc = "Bits 17:20 - hsic_ehci_lpsms_state"]
    #[inline(always)]
    pub fn hsic_ehci_lpsms_state(&self) -> HsicEhciLpsmsStateR {
        HsicEhciLpsmsStateR::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bit 21 - hsic_ehci_xfer_prdc"]
    #[inline(always)]
    pub fn hsic_ehci_xfer_prdc(&self) -> HsicEhciXferPrdcR {
        HsicEhciXferPrdcR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - hsic_ehci_xfer_cnt"]
    #[inline(always)]
    #[must_use]
    pub fn hsic_ehci_xfer_cnt(&mut self) -> HsicEhciXferCntW<GrfGrfHsicStatusSpec> {
        HsicEhciXferCntW::new(self, 0)
    }
    #[doc = "Bits 11:16 - hsic_ehci_usbsts"]
    #[inline(always)]
    #[must_use]
    pub fn hsic_ehci_usbsts(&mut self) -> HsicEhciUsbstsW<GrfGrfHsicStatusSpec> {
        HsicEhciUsbstsW::new(self, 11)
    }
    #[doc = "Bits 17:20 - hsic_ehci_lpsms_state"]
    #[inline(always)]
    #[must_use]
    pub fn hsic_ehci_lpsms_state(&mut self) -> HsicEhciLpsmsStateW<GrfGrfHsicStatusSpec> {
        HsicEhciLpsmsStateW::new(self, 17)
    }
    #[doc = "Bit 21 - hsic_ehci_xfer_prdc"]
    #[inline(always)]
    #[must_use]
    pub fn hsic_ehci_xfer_prdc(&mut self) -> HsicEhciXferPrdcW<GrfGrfHsicStatusSpec> {
        HsicEhciXferPrdcW::new(self, 21)
    }
}
#[doc = "hsic controller status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_grf_hsic_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_grf_hsic_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGrfHsicStatusSpec;
impl crate::RegisterSpec for GrfGrfHsicStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_grf_hsic_status::R`](R) reader structure"]
impl crate::Readable for GrfGrfHsicStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_grf_hsic_status::W`](W) writer structure"]
impl crate::Writable for GrfGrfHsicStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GRF_HSIC_STATUS to value 0"]
impl crate::Resettable for GrfGrfHsicStatusSpec {
    const RESET_VALUE: u32 = 0;
}

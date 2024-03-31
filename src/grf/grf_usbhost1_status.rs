#[doc = "Register `GRF_USBHOST1_STATUS` reader"]
pub type R = crate::R<GrfUsbhost1StatusSpec>;
#[doc = "Register `GRF_USBHOST1_STATUS` writer"]
pub type W = crate::W<GrfUsbhost1StatusSpec>;
#[doc = "Field `HOST1_EHCI_XFER_CNT` reader - host1_ehci_xfer_cnt"]
pub type Host1EhciXferCntR = crate::FieldReader<u16>;
#[doc = "Field `HOST1_EHCI_XFER_CNT` writer - host1_ehci_xfer_cnt"]
pub type Host1EhciXferCntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `HOST1_EHCI_USBSTS` reader - host1_ehci_usbsts"]
pub type Host1EhciUsbstsR = crate::FieldReader;
#[doc = "Field `HOST1_EHCI_USBSTS` writer - host1_ehci_usbsts"]
pub type Host1EhciUsbstsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `HOST1_EHCI_LPSMC_STATE` reader - host1_ehci_lpsmc_state"]
pub type Host1EhciLpsmcStateR = crate::FieldReader;
#[doc = "Field `HOST1_EHCI_LPSMC_STATE` writer - host1_ehci_lpsmc_state"]
pub type Host1EhciLpsmcStateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HOST1_OHCI_RWE` reader - host1_ohci_rwe"]
pub type Host1OhciRweR = crate::BitReader;
#[doc = "Field `HOST1_OHCI_RWE` writer - host1_ohci_rwe"]
pub type Host1OhciRweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST1_OHCI_RMTWKP` reader - host1_ohci_rmtwkp"]
pub type Host1OhciRmtwkpR = crate::BitReader;
#[doc = "Field `HOST1_OHCI_RMTWKP` writer - host1_ohci_rmtwkp"]
pub type Host1OhciRmtwkpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST1_OHCI_GLOBALSUSPEND` reader - host1_ohci_globalsuspend"]
pub type Host1OhciGlobalsuspendR = crate::BitReader;
#[doc = "Field `HOST1_OHCI_GLOBALSUSPEND` writer - host1_ohci_globalsuspend"]
pub type Host1OhciGlobalsuspendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST1_OHCI_DRWE` reader - host1_ohci_drwe"]
pub type Host1OhciDrweR = crate::BitReader;
#[doc = "Field `HOST1_OHCI_DRWE` writer - host1_ohci_drwe"]
pub type Host1OhciDrweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST1_OHCI_CCS` reader - host1_ohci_ccs"]
pub type Host1OhciCcsR = crate::BitReader;
#[doc = "Field `HOST1_OHCI_CCS` writer - host1_ohci_ccs"]
pub type Host1OhciCcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST1_OHCI_BUFACC` reader - host1_ohci_bufacc"]
pub type Host1OhciBufaccR = crate::BitReader;
#[doc = "Field `HOST1_OHCI_BUFACC` writer - host1_ohci_bufacc"]
pub type Host1OhciBufaccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST1_EHCI_XFER_PRDC` reader - host1_ehci_xfer_prdc"]
pub type Host1EhciXferPrdcR = crate::BitReader;
#[doc = "Field `HOST1_EHCI_XFER_PRDC` writer - host1_ehci_xfer_prdc"]
pub type Host1EhciXferPrdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST1_EHCI_BUFACC` reader - host1_ehci_bufacc"]
pub type Host1EhciBufaccR = crate::BitReader;
#[doc = "Field `HOST1_EHCI_BUFACC` writer - host1_ehci_bufacc"]
pub type Host1EhciBufaccW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - host1_ehci_xfer_cnt"]
    #[inline(always)]
    pub fn host1_ehci_xfer_cnt(&self) -> Host1EhciXferCntR {
        Host1EhciXferCntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:16 - host1_ehci_usbsts"]
    #[inline(always)]
    pub fn host1_ehci_usbsts(&self) -> Host1EhciUsbstsR {
        Host1EhciUsbstsR::new(((self.bits >> 11) & 0x3f) as u8)
    }
    #[doc = "Bits 17:20 - host1_ehci_lpsmc_state"]
    #[inline(always)]
    pub fn host1_ehci_lpsmc_state(&self) -> Host1EhciLpsmcStateR {
        Host1EhciLpsmcStateR::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bit 21 - host1_ohci_rwe"]
    #[inline(always)]
    pub fn host1_ohci_rwe(&self) -> Host1OhciRweR {
        Host1OhciRweR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - host1_ohci_rmtwkp"]
    #[inline(always)]
    pub fn host1_ohci_rmtwkp(&self) -> Host1OhciRmtwkpR {
        Host1OhciRmtwkpR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - host1_ohci_globalsuspend"]
    #[inline(always)]
    pub fn host1_ohci_globalsuspend(&self) -> Host1OhciGlobalsuspendR {
        Host1OhciGlobalsuspendR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - host1_ohci_drwe"]
    #[inline(always)]
    pub fn host1_ohci_drwe(&self) -> Host1OhciDrweR {
        Host1OhciDrweR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - host1_ohci_ccs"]
    #[inline(always)]
    pub fn host1_ohci_ccs(&self) -> Host1OhciCcsR {
        Host1OhciCcsR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - host1_ohci_bufacc"]
    #[inline(always)]
    pub fn host1_ohci_bufacc(&self) -> Host1OhciBufaccR {
        Host1OhciBufaccR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - host1_ehci_xfer_prdc"]
    #[inline(always)]
    pub fn host1_ehci_xfer_prdc(&self) -> Host1EhciXferPrdcR {
        Host1EhciXferPrdcR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - host1_ehci_bufacc"]
    #[inline(always)]
    pub fn host1_ehci_bufacc(&self) -> Host1EhciBufaccR {
        Host1EhciBufaccR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - host1_ehci_xfer_cnt"]
    #[inline(always)]
    #[must_use]
    pub fn host1_ehci_xfer_cnt(&mut self) -> Host1EhciXferCntW<GrfUsbhost1StatusSpec> {
        Host1EhciXferCntW::new(self, 0)
    }
    #[doc = "Bits 11:16 - host1_ehci_usbsts"]
    #[inline(always)]
    #[must_use]
    pub fn host1_ehci_usbsts(&mut self) -> Host1EhciUsbstsW<GrfUsbhost1StatusSpec> {
        Host1EhciUsbstsW::new(self, 11)
    }
    #[doc = "Bits 17:20 - host1_ehci_lpsmc_state"]
    #[inline(always)]
    #[must_use]
    pub fn host1_ehci_lpsmc_state(&mut self) -> Host1EhciLpsmcStateW<GrfUsbhost1StatusSpec> {
        Host1EhciLpsmcStateW::new(self, 17)
    }
    #[doc = "Bit 21 - host1_ohci_rwe"]
    #[inline(always)]
    #[must_use]
    pub fn host1_ohci_rwe(&mut self) -> Host1OhciRweW<GrfUsbhost1StatusSpec> {
        Host1OhciRweW::new(self, 21)
    }
    #[doc = "Bit 22 - host1_ohci_rmtwkp"]
    #[inline(always)]
    #[must_use]
    pub fn host1_ohci_rmtwkp(&mut self) -> Host1OhciRmtwkpW<GrfUsbhost1StatusSpec> {
        Host1OhciRmtwkpW::new(self, 22)
    }
    #[doc = "Bit 23 - host1_ohci_globalsuspend"]
    #[inline(always)]
    #[must_use]
    pub fn host1_ohci_globalsuspend(&mut self) -> Host1OhciGlobalsuspendW<GrfUsbhost1StatusSpec> {
        Host1OhciGlobalsuspendW::new(self, 23)
    }
    #[doc = "Bit 24 - host1_ohci_drwe"]
    #[inline(always)]
    #[must_use]
    pub fn host1_ohci_drwe(&mut self) -> Host1OhciDrweW<GrfUsbhost1StatusSpec> {
        Host1OhciDrweW::new(self, 24)
    }
    #[doc = "Bit 25 - host1_ohci_ccs"]
    #[inline(always)]
    #[must_use]
    pub fn host1_ohci_ccs(&mut self) -> Host1OhciCcsW<GrfUsbhost1StatusSpec> {
        Host1OhciCcsW::new(self, 25)
    }
    #[doc = "Bit 26 - host1_ohci_bufacc"]
    #[inline(always)]
    #[must_use]
    pub fn host1_ohci_bufacc(&mut self) -> Host1OhciBufaccW<GrfUsbhost1StatusSpec> {
        Host1OhciBufaccW::new(self, 26)
    }
    #[doc = "Bit 27 - host1_ehci_xfer_prdc"]
    #[inline(always)]
    #[must_use]
    pub fn host1_ehci_xfer_prdc(&mut self) -> Host1EhciXferPrdcW<GrfUsbhost1StatusSpec> {
        Host1EhciXferPrdcW::new(self, 27)
    }
    #[doc = "Bit 28 - host1_ehci_bufacc"]
    #[inline(always)]
    #[must_use]
    pub fn host1_ehci_bufacc(&mut self) -> Host1EhciBufaccW<GrfUsbhost1StatusSpec> {
        Host1EhciBufaccW::new(self, 28)
    }
}
#[doc = "usb host1 controller status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbhost1_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbhost1_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfUsbhost1StatusSpec;
impl crate::RegisterSpec for GrfUsbhost1StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_usbhost1_status::R`](R) reader structure"]
impl crate::Readable for GrfUsbhost1StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_usbhost1_status::W`](W) writer structure"]
impl crate::Writable for GrfUsbhost1StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_USBHOST1_STATUS to value 0"]
impl crate::Resettable for GrfUsbhost1StatusSpec {
    const RESET_VALUE: u32 = 0;
}

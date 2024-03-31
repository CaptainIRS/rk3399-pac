#[doc = "Register `GRF_USBHOST0_STATUS` reader"]
pub type R = crate::R<GrfUsbhost0StatusSpec>;
#[doc = "Register `GRF_USBHOST0_STATUS` writer"]
pub type W = crate::W<GrfUsbhost0StatusSpec>;
#[doc = "Field `HOST0_EHCI_XFER_CNT` reader - host0_ehci_xfer_cnt"]
pub type Host0EhciXferCntR = crate::FieldReader<u16>;
#[doc = "Field `HOST0_EHCI_XFER_CNT` writer - host0_ehci_xfer_cnt"]
pub type Host0EhciXferCntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `HOST0_EHCI_USBSTS` reader - host0_ehci_usbsts"]
pub type Host0EhciUsbstsR = crate::FieldReader;
#[doc = "Field `HOST0_EHCI_USBSTS` writer - host0_ehci_usbsts"]
pub type Host0EhciUsbstsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `HOST0_EHCI_LPSMC_STATE` reader - host0_ehci_lpsmc_state"]
pub type Host0EhciLpsmcStateR = crate::FieldReader;
#[doc = "Field `HOST0_EHCI_LPSMC_STATE` writer - host0_ehci_lpsmc_state"]
pub type Host0EhciLpsmcStateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HOST0_OHCI_RWE` reader - host0_ohci_rwe"]
pub type Host0OhciRweR = crate::BitReader;
#[doc = "Field `HOST0_OHCI_RWE` writer - host0_ohci_rwe"]
pub type Host0OhciRweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST0_OHCI_RMTWKP` reader - host0_ohci_rmtwkp"]
pub type Host0OhciRmtwkpR = crate::BitReader;
#[doc = "Field `HOST0_OHCI_RMTWKP` writer - host0_ohci_rmtwkp"]
pub type Host0OhciRmtwkpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST0_OHCI_GLOBALSUSPEND` reader - host0_ohci_globalsuspend"]
pub type Host0OhciGlobalsuspendR = crate::BitReader;
#[doc = "Field `HOST0_OHCI_GLOBALSUSPEND` writer - host0_ohci_globalsuspend"]
pub type Host0OhciGlobalsuspendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST0_OHCI_DRWE` reader - host0_ohci_drwe"]
pub type Host0OhciDrweR = crate::BitReader;
#[doc = "Field `HOST0_OHCI_DRWE` writer - host0_ohci_drwe"]
pub type Host0OhciDrweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST0_OHCI_CCS` reader - host0_ohci_ccs"]
pub type Host0OhciCcsR = crate::BitReader;
#[doc = "Field `HOST0_OHCI_CCS` writer - host0_ohci_ccs"]
pub type Host0OhciCcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST0_OHCI_BUFACC` reader - host0_ohci_bufacc"]
pub type Host0OhciBufaccR = crate::BitReader;
#[doc = "Field `HOST0_OHCI_BUFACC` writer - host0_ohci_bufacc"]
pub type Host0OhciBufaccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST0_EHCI_XFER_PRDC` reader - host0_ehci_xfer_prdc"]
pub type Host0EhciXferPrdcR = crate::BitReader;
#[doc = "Field `HOST0_EHCI_XFER_PRDC` writer - host0_ehci_xfer_prdc"]
pub type Host0EhciXferPrdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST0_EHCI_BUFACC` reader - host0_ehci_bufacc"]
pub type Host0EhciBufaccR = crate::BitReader;
#[doc = "Field `HOST0_EHCI_BUFACC` writer - host0_ehci_bufacc"]
pub type Host0EhciBufaccW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - host0_ehci_xfer_cnt"]
    #[inline(always)]
    pub fn host0_ehci_xfer_cnt(&self) -> Host0EhciXferCntR {
        Host0EhciXferCntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:16 - host0_ehci_usbsts"]
    #[inline(always)]
    pub fn host0_ehci_usbsts(&self) -> Host0EhciUsbstsR {
        Host0EhciUsbstsR::new(((self.bits >> 11) & 0x3f) as u8)
    }
    #[doc = "Bits 17:20 - host0_ehci_lpsmc_state"]
    #[inline(always)]
    pub fn host0_ehci_lpsmc_state(&self) -> Host0EhciLpsmcStateR {
        Host0EhciLpsmcStateR::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bit 21 - host0_ohci_rwe"]
    #[inline(always)]
    pub fn host0_ohci_rwe(&self) -> Host0OhciRweR {
        Host0OhciRweR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - host0_ohci_rmtwkp"]
    #[inline(always)]
    pub fn host0_ohci_rmtwkp(&self) -> Host0OhciRmtwkpR {
        Host0OhciRmtwkpR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - host0_ohci_globalsuspend"]
    #[inline(always)]
    pub fn host0_ohci_globalsuspend(&self) -> Host0OhciGlobalsuspendR {
        Host0OhciGlobalsuspendR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - host0_ohci_drwe"]
    #[inline(always)]
    pub fn host0_ohci_drwe(&self) -> Host0OhciDrweR {
        Host0OhciDrweR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - host0_ohci_ccs"]
    #[inline(always)]
    pub fn host0_ohci_ccs(&self) -> Host0OhciCcsR {
        Host0OhciCcsR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - host0_ohci_bufacc"]
    #[inline(always)]
    pub fn host0_ohci_bufacc(&self) -> Host0OhciBufaccR {
        Host0OhciBufaccR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - host0_ehci_xfer_prdc"]
    #[inline(always)]
    pub fn host0_ehci_xfer_prdc(&self) -> Host0EhciXferPrdcR {
        Host0EhciXferPrdcR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - host0_ehci_bufacc"]
    #[inline(always)]
    pub fn host0_ehci_bufacc(&self) -> Host0EhciBufaccR {
        Host0EhciBufaccR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - host0_ehci_xfer_cnt"]
    #[inline(always)]
    #[must_use]
    pub fn host0_ehci_xfer_cnt(&mut self) -> Host0EhciXferCntW<GrfUsbhost0StatusSpec> {
        Host0EhciXferCntW::new(self, 0)
    }
    #[doc = "Bits 11:16 - host0_ehci_usbsts"]
    #[inline(always)]
    #[must_use]
    pub fn host0_ehci_usbsts(&mut self) -> Host0EhciUsbstsW<GrfUsbhost0StatusSpec> {
        Host0EhciUsbstsW::new(self, 11)
    }
    #[doc = "Bits 17:20 - host0_ehci_lpsmc_state"]
    #[inline(always)]
    #[must_use]
    pub fn host0_ehci_lpsmc_state(&mut self) -> Host0EhciLpsmcStateW<GrfUsbhost0StatusSpec> {
        Host0EhciLpsmcStateW::new(self, 17)
    }
    #[doc = "Bit 21 - host0_ohci_rwe"]
    #[inline(always)]
    #[must_use]
    pub fn host0_ohci_rwe(&mut self) -> Host0OhciRweW<GrfUsbhost0StatusSpec> {
        Host0OhciRweW::new(self, 21)
    }
    #[doc = "Bit 22 - host0_ohci_rmtwkp"]
    #[inline(always)]
    #[must_use]
    pub fn host0_ohci_rmtwkp(&mut self) -> Host0OhciRmtwkpW<GrfUsbhost0StatusSpec> {
        Host0OhciRmtwkpW::new(self, 22)
    }
    #[doc = "Bit 23 - host0_ohci_globalsuspend"]
    #[inline(always)]
    #[must_use]
    pub fn host0_ohci_globalsuspend(&mut self) -> Host0OhciGlobalsuspendW<GrfUsbhost0StatusSpec> {
        Host0OhciGlobalsuspendW::new(self, 23)
    }
    #[doc = "Bit 24 - host0_ohci_drwe"]
    #[inline(always)]
    #[must_use]
    pub fn host0_ohci_drwe(&mut self) -> Host0OhciDrweW<GrfUsbhost0StatusSpec> {
        Host0OhciDrweW::new(self, 24)
    }
    #[doc = "Bit 25 - host0_ohci_ccs"]
    #[inline(always)]
    #[must_use]
    pub fn host0_ohci_ccs(&mut self) -> Host0OhciCcsW<GrfUsbhost0StatusSpec> {
        Host0OhciCcsW::new(self, 25)
    }
    #[doc = "Bit 26 - host0_ohci_bufacc"]
    #[inline(always)]
    #[must_use]
    pub fn host0_ohci_bufacc(&mut self) -> Host0OhciBufaccW<GrfUsbhost0StatusSpec> {
        Host0OhciBufaccW::new(self, 26)
    }
    #[doc = "Bit 27 - host0_ehci_xfer_prdc"]
    #[inline(always)]
    #[must_use]
    pub fn host0_ehci_xfer_prdc(&mut self) -> Host0EhciXferPrdcW<GrfUsbhost0StatusSpec> {
        Host0EhciXferPrdcW::new(self, 27)
    }
    #[doc = "Bit 28 - host0_ehci_bufacc"]
    #[inline(always)]
    #[must_use]
    pub fn host0_ehci_bufacc(&mut self) -> Host0EhciBufaccW<GrfUsbhost0StatusSpec> {
        Host0EhciBufaccW::new(self, 28)
    }
}
#[doc = "usb host0 controller status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbhost0_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbhost0_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfUsbhost0StatusSpec;
impl crate::RegisterSpec for GrfUsbhost0StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_usbhost0_status::R`](R) reader structure"]
impl crate::Readable for GrfUsbhost0StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_usbhost0_status::W`](W) writer structure"]
impl crate::Writable for GrfUsbhost0StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_USBHOST0_STATUS to value 0"]
impl crate::Resettable for GrfUsbhost0StatusSpec {
    const RESET_VALUE: u32 = 0;
}

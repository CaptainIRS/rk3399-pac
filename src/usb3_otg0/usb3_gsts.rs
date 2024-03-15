#[doc = "Register `USB3_GSTS` reader"]
pub type R = crate::R<Usb3GstsSpec>;
#[doc = "Register `USB3_GSTS` writer"]
pub type W = crate::W<Usb3GstsSpec>;
#[doc = "Field `CURMOD` reader - Current Mode of Operation Current Mode of Operation"]
pub type CurmodR = crate::FieldReader;
#[doc = "Field `BUSERRADDRVLD` reader - Bus Error Address Valid Indicates that the GBUSERRADDR register is valid and reports the first bus address that encounters a bus error."]
pub type BuserraddrvldR = crate::BitReader;
#[doc = "Field `BUSERRADDRVLD` writer - Bus Error Address Valid Indicates that the GBUSERRADDR register is valid and reports the first bus address that encounters a bus error."]
pub type BuserraddrvldW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CSRTIMEOUT` reader - CSR Timeout When this bit is 1'b1, it indicates that the software performed a write or read to a core register that could not be completed within DWC_USB3_CSR_ACCESS_TIMEOUT bus clock cycles (default: h1FFFF)."]
pub type CsrtimeoutR = crate::BitReader;
#[doc = "Field `CSRTIMEOUT` writer - CSR Timeout When this bit is 1'b1, it indicates that the software performed a write or read to a core register that could not be completed within DWC_USB3_CSR_ACCESS_TIMEOUT bus clock cycles (default: h1FFFF)."]
pub type CsrtimeoutW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DEVICE_IP` reader - Device Interrupt Pending This field indicates that there is a pending interrupt pertaining to peripheral (device) operation in the Device event queue."]
pub type DeviceIpR = crate::BitReader;
#[doc = "Field `HOST_IP` reader - Host Interrupt Pending This field indicates that there is a pending interrupt pertaining to xHC in the Host event queue."]
pub type HostIpR = crate::BitReader;
#[doc = "Field `ADP_IP` reader - ADP Interrupt Pending his field indicates that there is a pending interrupt pertaining to ADP in ADPEVT register."]
pub type AdpIpR = crate::BitReader;
#[doc = "Field `BC_IP` reader - Battery Charger Interrupt Pending This field indicates that there is a pending interrupt pertaining to BC in BCEVT register."]
pub type BcIpR = crate::BitReader;
#[doc = "Field `OTG_IP` reader - OTG Interrupt Pending This field indicates that there is a pending interrupt pertaining to OTG in OEVT register."]
pub type OtgIpR = crate::BitReader;
#[doc = "Field `SSIC_IP` reader - SSIC interrupt pending This field indicates that there is a pending interrupt related to SSIC in the SEVT register. Note: When the DWC_USB3_NUM_SSIC_PORTS parameter is set to zero, this bit is reserved."]
pub type SsicIpR = crate::BitReader;
#[doc = "Field `CBELT` reader - Current BELT Value In Host mode, this field indicates the minimum value of all received device BELT values and the BELT value that is set by the Set Latency Tolerance Value command."]
pub type CbeltR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:1 - Current Mode of Operation Current Mode of Operation"]
    #[inline(always)]
    pub fn curmod(&self) -> CurmodR {
        CurmodR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Bus Error Address Valid Indicates that the GBUSERRADDR register is valid and reports the first bus address that encounters a bus error."]
    #[inline(always)]
    pub fn buserraddrvld(&self) -> BuserraddrvldR {
        BuserraddrvldR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CSR Timeout When this bit is 1'b1, it indicates that the software performed a write or read to a core register that could not be completed within DWC_USB3_CSR_ACCESS_TIMEOUT bus clock cycles (default: h1FFFF)."]
    #[inline(always)]
    pub fn csrtimeout(&self) -> CsrtimeoutR {
        CsrtimeoutR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Device Interrupt Pending This field indicates that there is a pending interrupt pertaining to peripheral (device) operation in the Device event queue."]
    #[inline(always)]
    pub fn device_ip(&self) -> DeviceIpR {
        DeviceIpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Host Interrupt Pending This field indicates that there is a pending interrupt pertaining to xHC in the Host event queue."]
    #[inline(always)]
    pub fn host_ip(&self) -> HostIpR {
        HostIpR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADP Interrupt Pending his field indicates that there is a pending interrupt pertaining to ADP in ADPEVT register."]
    #[inline(always)]
    pub fn adp_ip(&self) -> AdpIpR {
        AdpIpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Battery Charger Interrupt Pending This field indicates that there is a pending interrupt pertaining to BC in BCEVT register."]
    #[inline(always)]
    pub fn bc_ip(&self) -> BcIpR {
        BcIpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - OTG Interrupt Pending This field indicates that there is a pending interrupt pertaining to OTG in OEVT register."]
    #[inline(always)]
    pub fn otg_ip(&self) -> OtgIpR {
        OtgIpR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SSIC interrupt pending This field indicates that there is a pending interrupt related to SSIC in the SEVT register. Note: When the DWC_USB3_NUM_SSIC_PORTS parameter is set to zero, this bit is reserved."]
    #[inline(always)]
    pub fn ssic_ip(&self) -> SsicIpR {
        SsicIpR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 20:31 - Current BELT Value In Host mode, this field indicates the minimum value of all received device BELT values and the BELT value that is set by the Set Latency Tolerance Value command."]
    #[inline(always)]
    pub fn cbelt(&self) -> CbeltR {
        CbeltR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 4 - Bus Error Address Valid Indicates that the GBUSERRADDR register is valid and reports the first bus address that encounters a bus error."]
    #[inline(always)]
    #[must_use]
    pub fn buserraddrvld(&mut self) -> BuserraddrvldW<Usb3GstsSpec> {
        BuserraddrvldW::new(self, 4)
    }
    #[doc = "Bit 5 - CSR Timeout When this bit is 1'b1, it indicates that the software performed a write or read to a core register that could not be completed within DWC_USB3_CSR_ACCESS_TIMEOUT bus clock cycles (default: h1FFFF)."]
    #[inline(always)]
    #[must_use]
    pub fn csrtimeout(&mut self) -> CsrtimeoutW<Usb3GstsSpec> {
        CsrtimeoutW::new(self, 5)
    }
}
#[doc = "Global Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3GstsSpec;
impl crate::RegisterSpec for Usb3GstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_gsts::R`](R) reader structure"]
impl crate::Readable for Usb3GstsSpec {}
#[doc = "`write(|w| ..)` method takes [`usb3_gsts::W`](W) writer structure"]
impl crate::Writable for Usb3GstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x30;
}
#[doc = "`reset()` method sets USB3_GSTS to value 0x7e80_0000"]
impl crate::Resettable for Usb3GstsSpec {
    const RESET_VALUE: u32 = 0x7e80_0000;
}

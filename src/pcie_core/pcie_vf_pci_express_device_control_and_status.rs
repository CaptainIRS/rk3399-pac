#[doc = "Register `PCIE_VF_PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS` reader"]
pub type R = crate::R<PcieVfPciExpressDeviceControlAndStatusSpec>;
#[doc = "Register `PCIE_VF_PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS` writer"]
pub type W = crate::W<PcieVfPciExpressDeviceControlAndStatusSpec>;
#[doc = "Field `ECER` reader - Enable Correctable Error Reporting \\[ECER\\]\n\nReserved"]
pub type EcerR = crate::BitReader;
#[doc = "Field `ENFER` reader - Enable Non- Fatal Error Reporting \\[ENFER\\]\n\nReserved"]
pub type EnferR = crate::BitReader;
#[doc = "Field `EFER` reader - Enable Fatal Error Reporting \\[EFER\\]\n\nReserved"]
pub type EferR = crate::BitReader;
#[doc = "Field `EURR` reader - Enable Unsupported Request Reporting \\[EURR\\]\n\nReserved"]
pub type EurrR = crate::BitReader;
#[doc = "Field `ERO` reader - Enable Relaxed Ordering \\[ERO\\]\n\nReserved"]
pub type EroR = crate::BitReader;
#[doc = "Field `MPS` reader - Max Payload Size \\[MPS\\]\n\nReserved"]
pub type MpsR = crate::FieldReader;
#[doc = "Field `ETFE` reader - Extended Tag Field Enable \\[ETFE\\]\n\nReserved"]
pub type EtfeR = crate::BitReader;
#[doc = "Field `EPF` reader - Enable Phantom Functions \\[EPF\\]\n\nReserved"]
pub type EpfR = crate::BitReader;
#[doc = "Field `EAP` reader - Enable Aux Power \\[EAP\\]\n\nReserved"]
pub type EapR = crate::BitReader;
#[doc = "Field `EBS` reader - Enable No Snoop \\[EBS\\]\n\nReserved"]
pub type EbsR = crate::BitReader;
#[doc = "Field `MRRS` reader - Max Read Request Size \\[MRRS\\]\n\nReserved"]
pub type MrrsR = crate::FieldReader;
#[doc = "Field `FLR` reader - Function- Level Reset \\[FLR\\]\n\nWriting a 1 into this bit position\n\ngenerated a Function-Level Reset\n\nfor the selected VF. This bit reads as\n\n0."]
pub type FlrR = crate::BitReader;
#[doc = "Field `FLR` writer - Function- Level Reset \\[FLR\\]\n\nWriting a 1 into this bit position\n\ngenerated a Function-Level Reset\n\nfor the selected VF. This bit reads as\n\n0."]
pub type FlrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CED` reader - Correctable Error Detected \\[CED\\]\n\nSet to 1 by the core when it detects\n\na correctable error, regardless of\n\nwhether error reporting is enabled\n\nor not, and regardless of whether\n\nthe error is masked"]
pub type CedR = crate::BitReader;
#[doc = "Field `CED` writer - Correctable Error Detected \\[CED\\]\n\nSet to 1 by the core when it detects\n\na correctable error, regardless of\n\nwhether error reporting is enabled\n\nor not, and regardless of whether\n\nthe error is masked"]
pub type CedW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `NFER` reader - Non-Fatal Error Detected \\[NFER\\]\n\nSet to 1 by the core when it detects\n\na non-fatal error, regardless of\n\nwhether error reporting is enabled\n\nor not, and regardless of whether\n\nthe error is masked."]
pub type NferR = crate::BitReader;
#[doc = "Field `NFER` writer - Non-Fatal Error Detected \\[NFER\\]\n\nSet to 1 by the core when it detects\n\na non-fatal error, regardless of\n\nwhether error reporting is enabled\n\nor not, and regardless of whether\n\nthe error is masked."]
pub type NferW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FED` reader - Fatal Error Detected \\[FED\\]\n\nSet to 1 by the core when it detects\n\na fatal error, regardless of whether\n\nerror reporting is enabled or not,\n\nand regardless of whether the error\n\nis masked."]
pub type FedR = crate::BitReader;
#[doc = "Field `FED` writer - Fatal Error Detected \\[FED\\]\n\nSet to 1 by the core when it detects\n\na fatal error, regardless of whether\n\nerror reporting is enabled or not,\n\nand regardless of whether the error\n\nis masked."]
pub type FedW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `URD` reader - Unsupported Request Detected \\[URD\\]\n\nSet to 1 by the core when it\n\nreceives an unsupported request,\n\nregardless of whether its reporting\n\nis enabled or not."]
pub type UrdR = crate::BitReader;
#[doc = "Field `URD` writer - Unsupported Request Detected \\[URD\\]\n\nSet to 1 by the core when it\n\nreceives an unsupported request,\n\nregardless of whether its reporting\n\nis enabled or not."]
pub type UrdW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `APD` reader - Aux Power Detected \\[APD\\]\n\nReserved"]
pub type ApdR = crate::BitReader;
#[doc = "Field `TP` reader - Transaction Pending \\[TP\\]\n\nIndicates if any of the Non-Posted\n\nrequests issued by the VF are still\n\npending."]
pub type TpR = crate::BitReader;
#[doc = "Field `R4` reader - Reserved \\[R4\\]\n\nReserved"]
pub type R4R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Enable Correctable Error Reporting \\[ECER\\]\n\nReserved"]
    #[inline(always)]
    pub fn ecer(&self) -> EcerR {
        EcerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Non- Fatal Error Reporting \\[ENFER\\]\n\nReserved"]
    #[inline(always)]
    pub fn enfer(&self) -> EnferR {
        EnferR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Fatal Error Reporting \\[EFER\\]\n\nReserved"]
    #[inline(always)]
    pub fn efer(&self) -> EferR {
        EferR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Unsupported Request Reporting \\[EURR\\]\n\nReserved"]
    #[inline(always)]
    pub fn eurr(&self) -> EurrR {
        EurrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Relaxed Ordering \\[ERO\\]\n\nReserved"]
    #[inline(always)]
    pub fn ero(&self) -> EroR {
        EroR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Max Payload Size \\[MPS\\]\n\nReserved"]
    #[inline(always)]
    pub fn mps(&self) -> MpsR {
        MpsR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Extended Tag Field Enable \\[ETFE\\]\n\nReserved"]
    #[inline(always)]
    pub fn etfe(&self) -> EtfeR {
        EtfeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Phantom Functions \\[EPF\\]\n\nReserved"]
    #[inline(always)]
    pub fn epf(&self) -> EpfR {
        EpfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Aux Power \\[EAP\\]\n\nReserved"]
    #[inline(always)]
    pub fn eap(&self) -> EapR {
        EapR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable No Snoop \\[EBS\\]\n\nReserved"]
    #[inline(always)]
    pub fn ebs(&self) -> EbsR {
        EbsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Max Read Request Size \\[MRRS\\]\n\nReserved"]
    #[inline(always)]
    pub fn mrrs(&self) -> MrrsR {
        MrrsR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Function- Level Reset \\[FLR\\]\n\nWriting a 1 into this bit position\n\ngenerated a Function-Level Reset\n\nfor the selected VF. This bit reads as\n\n0."]
    #[inline(always)]
    pub fn flr(&self) -> FlrR {
        FlrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Correctable Error Detected \\[CED\\]\n\nSet to 1 by the core when it detects\n\na correctable error, regardless of\n\nwhether error reporting is enabled\n\nor not, and regardless of whether\n\nthe error is masked"]
    #[inline(always)]
    pub fn ced(&self) -> CedR {
        CedR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Non-Fatal Error Detected \\[NFER\\]\n\nSet to 1 by the core when it detects\n\na non-fatal error, regardless of\n\nwhether error reporting is enabled\n\nor not, and regardless of whether\n\nthe error is masked."]
    #[inline(always)]
    pub fn nfer(&self) -> NferR {
        NferR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fatal Error Detected \\[FED\\]\n\nSet to 1 by the core when it detects\n\na fatal error, regardless of whether\n\nerror reporting is enabled or not,\n\nand regardless of whether the error\n\nis masked."]
    #[inline(always)]
    pub fn fed(&self) -> FedR {
        FedR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Unsupported Request Detected \\[URD\\]\n\nSet to 1 by the core when it\n\nreceives an unsupported request,\n\nregardless of whether its reporting\n\nis enabled or not."]
    #[inline(always)]
    pub fn urd(&self) -> UrdR {
        UrdR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Aux Power Detected \\[APD\\]\n\nReserved"]
    #[inline(always)]
    pub fn apd(&self) -> ApdR {
        ApdR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transaction Pending \\[TP\\]\n\nIndicates if any of the Non-Posted\n\nrequests issued by the VF are still\n\npending."]
    #[inline(always)]
    pub fn tp(&self) -> TpR {
        TpR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:31 - Reserved \\[R4\\]\n\nReserved"]
    #[inline(always)]
    pub fn r4(&self) -> R4R {
        R4R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 15 - Function- Level Reset \\[FLR\\]\n\nWriting a 1 into this bit position\n\ngenerated a Function-Level Reset\n\nfor the selected VF. This bit reads as\n\n0."]
    #[inline(always)]
    #[must_use]
    pub fn flr(&mut self) -> FlrW<PcieVfPciExpressDeviceControlAndStatusSpec> {
        FlrW::new(self, 15)
    }
    #[doc = "Bit 16 - Correctable Error Detected \\[CED\\]\n\nSet to 1 by the core when it detects\n\na correctable error, regardless of\n\nwhether error reporting is enabled\n\nor not, and regardless of whether\n\nthe error is masked"]
    #[inline(always)]
    #[must_use]
    pub fn ced(&mut self) -> CedW<PcieVfPciExpressDeviceControlAndStatusSpec> {
        CedW::new(self, 16)
    }
    #[doc = "Bit 17 - Non-Fatal Error Detected \\[NFER\\]\n\nSet to 1 by the core when it detects\n\na non-fatal error, regardless of\n\nwhether error reporting is enabled\n\nor not, and regardless of whether\n\nthe error is masked."]
    #[inline(always)]
    #[must_use]
    pub fn nfer(&mut self) -> NferW<PcieVfPciExpressDeviceControlAndStatusSpec> {
        NferW::new(self, 17)
    }
    #[doc = "Bit 18 - Fatal Error Detected \\[FED\\]\n\nSet to 1 by the core when it detects\n\na fatal error, regardless of whether\n\nerror reporting is enabled or not,\n\nand regardless of whether the error\n\nis masked."]
    #[inline(always)]
    #[must_use]
    pub fn fed(&mut self) -> FedW<PcieVfPciExpressDeviceControlAndStatusSpec> {
        FedW::new(self, 18)
    }
    #[doc = "Bit 19 - Unsupported Request Detected \\[URD\\]\n\nSet to 1 by the core when it\n\nreceives an unsupported request,\n\nregardless of whether its reporting\n\nis enabled or not."]
    #[inline(always)]
    #[must_use]
    pub fn urd(&mut self) -> UrdW<PcieVfPciExpressDeviceControlAndStatusSpec> {
        UrdW::new(self, 19)
    }
}
#[doc = "PCI Express Device Control and Status Register\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_pci_express_device_control_and_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_vf_pci_express_device_control_and_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfPciExpressDeviceControlAndStatusSpec;
impl crate::RegisterSpec for PcieVfPciExpressDeviceControlAndStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_pci_express_device_control_and_status::R`](R) reader structure"]
impl crate::Readable for PcieVfPciExpressDeviceControlAndStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_vf_pci_express_device_control_and_status::W`](W) writer structure"]
impl crate::Writable for PcieVfPciExpressDeviceControlAndStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x000f_0000;
}
#[doc = "`reset()` method sets PCIE_VF_PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS to value 0"]
impl crate::Resettable for PcieVfPciExpressDeviceControlAndStatusSpec {
    const RESET_VALUE: u32 = 0;
}

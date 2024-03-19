#[doc = "Register `PCIE_PF_INTERRUPT_LINE_AND_INTERRUPT_PIN` reader"]
pub type R = crate::R<PciePfInterruptLineAndInterruptPinSpec>;
#[doc = "Register `PCIE_PF_INTERRUPT_LINE_AND_INTERRUPT_PIN` writer"]
pub type W = crate::W<PciePfInterruptLineAndInterruptPinSpec>;
#[doc = "Field `ILR` reader - Interrupt Line Register \\[ILR\\]\n\nIdentifies the IRQx input of the\n\ninterrupt controller at the Root\n\nComplex that is activated by this\n\nFunctions interrupt (00 = IRQ0, ... ,\n\n0F = IRQ15, FF = unknown or not\n\nconnected). This field is writable\n\nfrom the local management bus."]
pub type IlrR = crate::FieldReader;
#[doc = "Field `ILR` writer - Interrupt Line Register \\[ILR\\]\n\nIdentifies the IRQx input of the\n\ninterrupt controller at the Root\n\nComplex that is activated by this\n\nFunctions interrupt (00 = IRQ0, ... ,\n\n0F = IRQ15, FF = unknown or not\n\nconnected). This field is writable\n\nfrom the local management bus."]
pub type IlrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IPR` reader - Interrupt Pin Register \\[IPR\\]\n\nIdentifies the interrupt input (A, B,\n\nC, D) to which this Functions\n\ninterrupt output is connected to\n\n(01= INTA, 02 = INTB, 03 = INTC,\n\n04 = INTD). The assignment of\n\ninterrupt inputs to Functions is fixed\n\nwhen the core is configured. This\n\nfield can be re-written independently\n\nfor each Function from the local\n\nmanagement bus."]
pub type IprR = crate::FieldReader;
#[doc = "Field `R16` reader - Reserved \\[R16\\]\n\nReserved"]
pub type R16R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - Interrupt Line Register \\[ILR\\]\n\nIdentifies the IRQx input of the\n\ninterrupt controller at the Root\n\nComplex that is activated by this\n\nFunctions interrupt (00 = IRQ0, ... ,\n\n0F = IRQ15, FF = unknown or not\n\nconnected). This field is writable\n\nfrom the local management bus."]
    #[inline(always)]
    pub fn ilr(&self) -> IlrR {
        IlrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Interrupt Pin Register \\[IPR\\]\n\nIdentifies the interrupt input (A, B,\n\nC, D) to which this Functions\n\ninterrupt output is connected to\n\n(01= INTA, 02 = INTB, 03 = INTC,\n\n04 = INTD). The assignment of\n\ninterrupt inputs to Functions is fixed\n\nwhen the core is configured. This\n\nfield can be re-written independently\n\nfor each Function from the local\n\nmanagement bus."]
    #[inline(always)]
    pub fn ipr(&self) -> IprR {
        IprR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:31 - Reserved \\[R16\\]\n\nReserved"]
    #[inline(always)]
    pub fn r16(&self) -> R16R {
        R16R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt Line Register \\[ILR\\]\n\nIdentifies the IRQx input of the\n\ninterrupt controller at the Root\n\nComplex that is activated by this\n\nFunctions interrupt (00 = IRQ0, ... ,\n\n0F = IRQ15, FF = unknown or not\n\nconnected). This field is writable\n\nfrom the local management bus."]
    #[inline(always)]
    #[must_use]
    pub fn ilr(&mut self) -> IlrW<PciePfInterruptLineAndInterruptPinSpec> {
        IlrW::new(self, 0)
    }
}
#[doc = "Interrupt Line and Interrupt Pin Register\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_interrupt_line_and_interrupt_pin::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_interrupt_line_and_interrupt_pin::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfInterruptLineAndInterruptPinSpec;
impl crate::RegisterSpec for PciePfInterruptLineAndInterruptPinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_interrupt_line_and_interrupt_pin::R`](R) reader structure"]
impl crate::Readable for PciePfInterruptLineAndInterruptPinSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_pf_interrupt_line_and_interrupt_pin::W`](W) writer structure"]
impl crate::Writable for PciePfInterruptLineAndInterruptPinSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_PF_INTERRUPT_LINE_AND_INTERRUPT_PIN to value 0x01ff"]
impl crate::Resettable for PciePfInterruptLineAndInterruptPinSpec {
    const RESET_VALUE: u32 = 0x01ff;
}

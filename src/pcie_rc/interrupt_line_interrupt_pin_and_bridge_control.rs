#[doc = "Register `INTERRUPT_LINE_INTERRUPT_PIN_AND_BRIDGE_CONTROL` reader"]
pub type R = crate::R<InterruptLineInterruptPinAndBridgeControlSpec>;
#[doc = "Register `INTERRUPT_LINE_INTERRUPT_PIN_AND_BRIDGE_CONTROL` writer"]
pub type W = crate::W<InterruptLineInterruptPinAndBridgeControlSpec>;
#[doc = "Field `ILR` reader - Interrupt Line Register \\[ILR\\]
This field can be read and written from the local management bus, but its value is not used within the core. The given reset value is for PF0."]
pub type IlrR = crate::FieldReader;
#[doc = "Field `ILR` writer - Interrupt Line Register \\[ILR\\]
This field can be read and written from the local management bus, but its value is not used within the core. The given reset value is for PF0."]
pub type IlrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IPR` reader - Interrupt Pin Register \\[IPR\\]
Identifies the interrupt input (A, B, C, D) to which this Functions interrupt output is connected to (01= INTA, 02 = INTB, 03 = INTC, 04 = INTD). The assignment of interrupt inputs to Functions is fixed when the core is configured. This field can be re- written independently for each Function from the local management bus. Default values - PF0: 01 (INTA), PF1: 02 (INTB)."]
pub type IprR = crate::FieldReader;
#[doc = "Field `R5` reader - Reserved \\[R5\\]
Reserved"]
pub type R5R = crate::FieldReader;
#[doc = "Field `PERE` reader - Parity Error Response Enable \\[PERE\\]
This field can be read and written from the local management APB bus. It is used only to enable the Master Data Parity Error bit in the Secondary Status Register."]
pub type PereR = crate::BitReader;
#[doc = "Field `PERE` writer - Parity Error Response Enable \\[PERE\\]
This field can be read and written from the local management APB bus. It is used only to enable the Master Data Parity Error bit in the Secondary Status Register."]
pub type PereW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCSE` reader - Bridge Control SERR Enable \\[BCSE\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
pub type BcseR = crate::BitReader;
#[doc = "Field `BCSE` writer - Bridge Control SERR Enable \\[BCSE\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
pub type BcseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISAE` reader - ISA Enable \\[ISAE\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
pub type IsaeR = crate::BitReader;
#[doc = "Field `ISAE` writer - ISA Enable \\[ISAE\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
pub type IsaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VGAE` reader - VGA Enable \\[VGAE\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
pub type VgaeR = crate::BitReader;
#[doc = "Field `VGAE` writer - VGA Enable \\[VGAE\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
pub type VgaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VGA16D` reader - VGA 16 DEcode \\[VGA16D\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
pub type Vga16dR = crate::BitReader;
#[doc = "Field `VGA16D` writer - VGA 16 DEcode \\[VGA16D\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
pub type Vga16dW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R21` reader - Reserved \\[R21\\]
Reserved"]
pub type R21R = crate::BitReader;
#[doc = "Field `BCRSBR` reader - Bridge Control Register Secondary Bus Reset \\[BCRSBR\\]
This field can be read and written from the local management APB bus. When set, it initiates a hot reset on the link."]
pub type BcrsbrR = crate::BitReader;
#[doc = "Field `BCRSBR` writer - Bridge Control Register Secondary Bus Reset \\[BCRSBR\\]
This field can be read and written from the local management APB bus. When set, it initiates a hot reset on the link."]
pub type BcrsbrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R23` reader - Reserved \\[R23\\]
Reserved"]
pub type R23R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - Interrupt Line Register \\[ILR\\]
This field can be read and written from the local management bus, but its value is not used within the core. The given reset value is for PF0."]
    #[inline(always)]
    pub fn ilr(&self) -> IlrR {
        IlrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Interrupt Pin Register \\[IPR\\]
Identifies the interrupt input (A, B, C, D) to which this Functions interrupt output is connected to (01= INTA, 02 = INTB, 03 = INTC, 04 = INTD). The assignment of interrupt inputs to Functions is fixed when the core is configured. This field can be re- written independently for each Function from the local management bus. Default values - PF0: 01 (INTA), PF1: 02 (INTB)."]
    #[inline(always)]
    pub fn ipr(&self) -> IprR {
        IprR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - Reserved \\[R5\\]
Reserved"]
    #[inline(always)]
    pub fn r5(&self) -> R5R {
        R5R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Parity Error Response Enable \\[PERE\\]
This field can be read and written from the local management APB bus. It is used only to enable the Master Data Parity Error bit in the Secondary Status Register."]
    #[inline(always)]
    pub fn pere(&self) -> PereR {
        PereR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bridge Control SERR Enable \\[BCSE\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
    #[inline(always)]
    pub fn bcse(&self) -> BcseR {
        BcseR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ISA Enable \\[ISAE\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
    #[inline(always)]
    pub fn isae(&self) -> IsaeR {
        IsaeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - VGA Enable \\[VGAE\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
    #[inline(always)]
    pub fn vgae(&self) -> VgaeR {
        VgaeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - VGA 16 DEcode \\[VGA16D\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
    #[inline(always)]
    pub fn vga16d(&self) -> Vga16dR {
        Vga16dR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reserved \\[R21\\]
Reserved"]
    #[inline(always)]
    pub fn r21(&self) -> R21R {
        R21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Bridge Control Register Secondary Bus Reset \\[BCRSBR\\]
This field can be read and written from the local management APB bus. When set, it initiates a hot reset on the link."]
    #[inline(always)]
    pub fn bcrsbr(&self) -> BcrsbrR {
        BcrsbrR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:31 - Reserved \\[R23\\]
Reserved"]
    #[inline(always)]
    pub fn r23(&self) -> R23R {
        R23R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt Line Register \\[ILR\\]
This field can be read and written from the local management bus, but its value is not used within the core. The given reset value is for PF0."]
    #[inline(always)]
    #[must_use]
    pub fn ilr(&mut self) -> IlrW<InterruptLineInterruptPinAndBridgeControlSpec> {
        IlrW::new(self, 0)
    }
    #[doc = "Bit 16 - Parity Error Response Enable \\[PERE\\]
This field can be read and written from the local management APB bus. It is used only to enable the Master Data Parity Error bit in the Secondary Status Register."]
    #[inline(always)]
    #[must_use]
    pub fn pere(&mut self) -> PereW<InterruptLineInterruptPinAndBridgeControlSpec> {
        PereW::new(self, 16)
    }
    #[doc = "Bit 17 - Bridge Control SERR Enable \\[BCSE\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
    #[inline(always)]
    #[must_use]
    pub fn bcse(&mut self) -> BcseW<InterruptLineInterruptPinAndBridgeControlSpec> {
        BcseW::new(self, 17)
    }
    #[doc = "Bit 18 - ISA Enable \\[ISAE\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
    #[inline(always)]
    #[must_use]
    pub fn isae(&mut self) -> IsaeW<InterruptLineInterruptPinAndBridgeControlSpec> {
        IsaeW::new(self, 18)
    }
    #[doc = "Bit 19 - VGA Enable \\[VGAE\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
    #[inline(always)]
    #[must_use]
    pub fn vgae(&mut self) -> VgaeW<InterruptLineInterruptPinAndBridgeControlSpec> {
        VgaeW::new(self, 19)
    }
    #[doc = "Bit 20 - VGA 16 DEcode \\[VGA16D\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
    #[inline(always)]
    #[must_use]
    pub fn vga16d(&mut self) -> Vga16dW<InterruptLineInterruptPinAndBridgeControlSpec> {
        Vga16dW::new(self, 20)
    }
    #[doc = "Bit 22 - Bridge Control Register Secondary Bus Reset \\[BCRSBR\\]
This field can be read and written from the local management APB bus. When set, it initiates a hot reset on the link."]
    #[inline(always)]
    #[must_use]
    pub fn bcrsbr(&mut self) -> BcrsbrW<InterruptLineInterruptPinAndBridgeControlSpec> {
        BcrsbrW::new(self, 22)
    }
}
#[doc = "Interrupt Line, Interrupt Pin Register and Bridge Control Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_line_interrupt_pin_and_bridge_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_line_interrupt_pin_and_bridge_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptLineInterruptPinAndBridgeControlSpec;
impl crate::RegisterSpec for InterruptLineInterruptPinAndBridgeControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_line_interrupt_pin_and_bridge_control::R`](R) reader structure"]
impl crate::Readable for InterruptLineInterruptPinAndBridgeControlSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt_line_interrupt_pin_and_bridge_control::W`](W) writer structure"]
impl crate::Writable for InterruptLineInterruptPinAndBridgeControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTERRUPT_LINE_INTERRUPT_PIN_AND_BRIDGE_CONTROL to value 0x01ff"]
impl crate::Resettable for InterruptLineInterruptPinAndBridgeControlSpec {
    const RESET_VALUE: u32 = 0x01ff;
}

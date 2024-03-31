#[doc = "Register `PORTA_EOI` writer"]
pub type W = crate::W<PortaEoiSpec>;
#[doc = "Controls the clearing of edge type interrupts from Port A. When a\n\n1 is written into a corresponding bit of this register, the interrupt\n\nis cleared. All interrupts are cleared when Port A is not configured\n\nfor interrupts.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GpioPortaEoi {
    #[doc = "0: No interrupt clear (default)"]
    B0 = 0,
    #[doc = "1: Clear interrupt"]
    B1 = 1,
}
impl From<GpioPortaEoi> for u32 {
    #[inline(always)]
    fn from(variant: GpioPortaEoi) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GpioPortaEoi {
    type Ux = u32;
}
#[doc = "Field `GPIO_PORTA_EOI` writer - Controls the clearing of edge type interrupts from Port A. When a\n\n1 is written into a corresponding bit of this register, the interrupt\n\nis cleared. All interrupts are cleared when Port A is not configured\n\nfor interrupts."]
pub type GpioPortaEoiW<'a, REG> = crate::FieldWriter<'a, REG, 32, GpioPortaEoi>;
impl<'a, REG> GpioPortaEoiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "No interrupt clear (default)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GpioPortaEoi::B0)
    }
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GpioPortaEoi::B1)
    }
}
impl W {
    #[doc = "Bits 0:31 - Controls the clearing of edge type interrupts from Port A. When a\n\n1 is written into a corresponding bit of this register, the interrupt\n\nis cleared. All interrupts are cleared when Port A is not configured\n\nfor interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_porta_eoi(&mut self) -> GpioPortaEoiW<PortaEoiSpec> {
        GpioPortaEoiW::new(self, 0)
    }
}
#[doc = "Port A clear interrupt register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`porta_eoi::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PortaEoiSpec;
impl crate::RegisterSpec for PortaEoiSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`porta_eoi::W`](W) writer structure"]
impl crate::Writable for PortaEoiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PORTA_EOI to value 0"]
impl crate::Resettable for PortaEoiSpec {
    const RESET_VALUE: u32 = 0;
}

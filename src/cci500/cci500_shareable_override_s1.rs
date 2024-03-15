#[doc = "Register `CCI500_SHAREABLE_OVERRIDE_S1` reader"]
pub type R = crate::R<Cci500ShareableOverrideS1Spec>;
#[doc = "Register `CCI500_SHAREABLE_OVERRIDE_S1` writer"]
pub type W = crate::W<Cci500ShareableOverrideS1Spec>;
#[doc = "Field `DOMAIN_OVERRIDE` reader - Shareable override for slave interface: 0b00-0b01: Do not override AxDOMAIN inputs. 0b10: Override AxDOMAIN inputs to 0b00, meaning that all transactions are treated as non-shareable: ReadOnce becomes ReadNoSnoop. WriteUnique and WriteLineUnique become WriteNoSnoop. CleanShared, CleanInvalid, and MakeInvalid transactions do not generate snoops. 0b11 Override AxDOMAIN inputs to 0b01, meaning that all Normal transactions are treated as shareable: ReadNoSnoop becomes ReadOnce. WriteNoSnoop becomes WriteUnique. CleanShared, CleanInvalid, and MakeInvalid transactions generate snoops"]
pub type DomainOverrideR = crate::FieldReader;
#[doc = "Field `DOMAIN_OVERRIDE` writer - Shareable override for slave interface: 0b00-0b01: Do not override AxDOMAIN inputs. 0b10: Override AxDOMAIN inputs to 0b00, meaning that all transactions are treated as non-shareable: ReadOnce becomes ReadNoSnoop. WriteUnique and WriteLineUnique become WriteNoSnoop. CleanShared, CleanInvalid, and MakeInvalid transactions do not generate snoops. 0b11 Override AxDOMAIN inputs to 0b01, meaning that all Normal transactions are treated as shareable: ReadNoSnoop becomes ReadOnce. WriteNoSnoop becomes WriteUnique. CleanShared, CleanInvalid, and MakeInvalid transactions generate snoops"]
pub type DomainOverrideW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Shareable override for slave interface: 0b00-0b01: Do not override AxDOMAIN inputs. 0b10: Override AxDOMAIN inputs to 0b00, meaning that all transactions are treated as non-shareable: ReadOnce becomes ReadNoSnoop. WriteUnique and WriteLineUnique become WriteNoSnoop. CleanShared, CleanInvalid, and MakeInvalid transactions do not generate snoops. 0b11 Override AxDOMAIN inputs to 0b01, meaning that all Normal transactions are treated as shareable: ReadNoSnoop becomes ReadOnce. WriteNoSnoop becomes WriteUnique. CleanShared, CleanInvalid, and MakeInvalid transactions generate snoops"]
    #[inline(always)]
    pub fn domain_override(&self) -> DomainOverrideR {
        DomainOverrideR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Shareable override for slave interface: 0b00-0b01: Do not override AxDOMAIN inputs. 0b10: Override AxDOMAIN inputs to 0b00, meaning that all transactions are treated as non-shareable: ReadOnce becomes ReadNoSnoop. WriteUnique and WriteLineUnique become WriteNoSnoop. CleanShared, CleanInvalid, and MakeInvalid transactions do not generate snoops. 0b11 Override AxDOMAIN inputs to 0b01, meaning that all Normal transactions are treated as shareable: ReadNoSnoop becomes ReadOnce. WriteNoSnoop becomes WriteUnique. CleanShared, CleanInvalid, and MakeInvalid transactions generate snoops"]
    #[inline(always)]
    #[must_use]
    pub fn domain_override(&mut self) -> DomainOverrideW<Cci500ShareableOverrideS1Spec> {
        DomainOverrideW::new(self, 0)
    }
}
#[doc = "Shareable Override Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci500_shareable_override_s1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci500_shareable_override_s1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cci500ShareableOverrideS1Spec;
impl crate::RegisterSpec for Cci500ShareableOverrideS1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cci500_shareable_override_s1::R`](R) reader structure"]
impl crate::Readable for Cci500ShareableOverrideS1Spec {}
#[doc = "`write(|w| ..)` method takes [`cci500_shareable_override_s1::W`](W) writer structure"]
impl crate::Writable for Cci500ShareableOverrideS1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCI500_SHAREABLE_OVERRIDE_S1 to value 0"]
impl crate::Resettable for Cci500ShareableOverrideS1Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `SHAREABLE_OVERRIDE_S1` reader"]
pub type R = crate::R<ShareableOverrideS1Spec>;
#[doc = "Register `SHAREABLE_OVERRIDE_S1` writer"]
pub type W = crate::W<ShareableOverrideS1Spec>;
#[doc = "Field `DOMAIN_OVERRIDE` reader - Shareable override for slave interface:\n\n0b00-0b01: Do not override AxDOMAIN\n\ninputs.\n\n0b10: Override AxDOMAIN inputs to 0b00,\n\nmeaning that all\n\ntransactions are treated as non-shareable:\n\nReadOnce becomes ReadNoSnoop.\n\nWriteUnique and WriteLineUnique become\n\nWriteNoSnoop.\n\nCleanShared, CleanInvalid, and\n\nMakeInvalid transactions do not generate\n\nsnoops.\n\n0b11 Override AxDOMAIN inputs to 0b01,\n\nmeaning that all Normal\n\ntransactions are treated as shareable:\n\nReadNoSnoop becomes ReadOnce.\n\nWriteNoSnoop becomes WriteUnique.\n\nCleanShared, CleanInvalid, and MakeInvalid\n\ntransactions generate snoops"]
pub type DomainOverrideR = crate::FieldReader;
#[doc = "Field `DOMAIN_OVERRIDE` writer - Shareable override for slave interface:\n\n0b00-0b01: Do not override AxDOMAIN\n\ninputs.\n\n0b10: Override AxDOMAIN inputs to 0b00,\n\nmeaning that all\n\ntransactions are treated as non-shareable:\n\nReadOnce becomes ReadNoSnoop.\n\nWriteUnique and WriteLineUnique become\n\nWriteNoSnoop.\n\nCleanShared, CleanInvalid, and\n\nMakeInvalid transactions do not generate\n\nsnoops.\n\n0b11 Override AxDOMAIN inputs to 0b01,\n\nmeaning that all Normal\n\ntransactions are treated as shareable:\n\nReadNoSnoop becomes ReadOnce.\n\nWriteNoSnoop becomes WriteUnique.\n\nCleanShared, CleanInvalid, and MakeInvalid\n\ntransactions generate snoops"]
pub type DomainOverrideW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Shareable override for slave interface:\n\n0b00-0b01: Do not override AxDOMAIN\n\ninputs.\n\n0b10: Override AxDOMAIN inputs to 0b00,\n\nmeaning that all\n\ntransactions are treated as non-shareable:\n\nReadOnce becomes ReadNoSnoop.\n\nWriteUnique and WriteLineUnique become\n\nWriteNoSnoop.\n\nCleanShared, CleanInvalid, and\n\nMakeInvalid transactions do not generate\n\nsnoops.\n\n0b11 Override AxDOMAIN inputs to 0b01,\n\nmeaning that all Normal\n\ntransactions are treated as shareable:\n\nReadNoSnoop becomes ReadOnce.\n\nWriteNoSnoop becomes WriteUnique.\n\nCleanShared, CleanInvalid, and MakeInvalid\n\ntransactions generate snoops"]
    #[inline(always)]
    pub fn domain_override(&self) -> DomainOverrideR {
        DomainOverrideR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Shareable override for slave interface:\n\n0b00-0b01: Do not override AxDOMAIN\n\ninputs.\n\n0b10: Override AxDOMAIN inputs to 0b00,\n\nmeaning that all\n\ntransactions are treated as non-shareable:\n\nReadOnce becomes ReadNoSnoop.\n\nWriteUnique and WriteLineUnique become\n\nWriteNoSnoop.\n\nCleanShared, CleanInvalid, and\n\nMakeInvalid transactions do not generate\n\nsnoops.\n\n0b11 Override AxDOMAIN inputs to 0b01,\n\nmeaning that all Normal\n\ntransactions are treated as shareable:\n\nReadNoSnoop becomes ReadOnce.\n\nWriteNoSnoop becomes WriteUnique.\n\nCleanShared, CleanInvalid, and MakeInvalid\n\ntransactions generate snoops"]
    #[inline(always)]
    #[must_use]
    pub fn domain_override(&mut self) -> DomainOverrideW<ShareableOverrideS1Spec> {
        DomainOverrideW::new(self, 0)
    }
}
#[doc = "Shareable Override Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shareable_override_s1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shareable_override_s1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShareableOverrideS1Spec;
impl crate::RegisterSpec for ShareableOverrideS1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shareable_override_s1::R`](R) reader structure"]
impl crate::Readable for ShareableOverrideS1Spec {}
#[doc = "`write(|w| ..)` method takes [`shareable_override_s1::W`](W) writer structure"]
impl crate::Writable for ShareableOverrideS1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHAREABLE_OVERRIDE_S1 to value 0"]
impl crate::Resettable for ShareableOverrideS1Spec {
    const RESET_VALUE: u32 = 0;
}

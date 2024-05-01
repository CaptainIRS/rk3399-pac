#[doc = "Register `PKA_M` reader"]
pub type R = crate::R<PkaMSpec>;
#[doc = "Register `PKA_M` writer"]
pub type W = crate::W<PkaMSpec>;
#[doc = "Field `M` reader - PKA input or output data.\n\nPKA result = (M ^ E) mod N.\n\nWhen it finishes, the result data is in M position. Start from PKA_M\n\nbase address, and may contain 512/1024/2048 bits data."]
pub type MR = crate::FieldReader<u32>;
#[doc = "Field `M` writer - PKA input or output data.\n\nPKA result = (M ^ E) mod N.\n\nWhen it finishes, the result data is in M position. Start from PKA_M\n\nbase address, and may contain 512/1024/2048 bits data."]
pub type MW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PKA input or output data.\n\nPKA result = (M ^ E) mod N.\n\nWhen it finishes, the result data is in M position. Start from PKA_M\n\nbase address, and may contain 512/1024/2048 bits data."]
    #[inline(always)]
    pub fn m(&self) -> MR {
        MR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PKA input or output data.\n\nPKA result = (M ^ E) mod N.\n\nWhen it finishes, the result data is in M position. Start from PKA_M\n\nbase address, and may contain 512/1024/2048 bits data."]
    #[inline(always)]
    #[must_use]
    pub fn m(&mut self) -> MW<PkaMSpec> {
        MW::new(self, 0)
    }
}
#[doc = "PKA input/output data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pka_m::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pka_m::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkaMSpec;
impl crate::RegisterSpec for PkaMSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pka_m::R`](R) reader structure"]
impl crate::Readable for PkaMSpec {}
#[doc = "`write(|w| ..)` method takes [`pka_m::W`](W) writer structure"]
impl crate::Writable for PkaMSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKA_M to value 0"]
impl crate::Resettable for PkaMSpec {
    const RESET_VALUE: u32 = 0;
}

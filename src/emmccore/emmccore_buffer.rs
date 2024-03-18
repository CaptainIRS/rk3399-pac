#[doc = "Register `EMMCCORE_BUFFER` reader"]
pub type R = crate::R<EmmccoreBufferSpec>;
#[doc = "Register `EMMCCORE_BUFFER` writer"]
pub type W = crate::W<EmmccoreBufferSpec>;
#[doc = "Field `BUFFERDATA` reader - The Host Controller Buffer can be accessed through this 32-bit Data Port Register."]
pub type BufferdataR = crate::FieldReader<u32>;
#[doc = "Field `BUFFERDATA` writer - The Host Controller Buffer can be accessed through this 32-bit Data Port Register."]
pub type BufferdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The Host Controller Buffer can be accessed through this 32-bit Data Port Register."]
    #[inline(always)]
    pub fn bufferdata(&self) -> BufferdataR {
        BufferdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The Host Controller Buffer can be accessed through this 32-bit Data Port Register."]
    #[inline(always)]
    #[must_use]
    pub fn bufferdata(&mut self) -> BufferdataW<EmmccoreBufferSpec> {
        BufferdataW::new(self, 0)
    }
}
#[doc = "Buffer data port register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_buffer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_buffer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreBufferSpec;
impl crate::RegisterSpec for EmmccoreBufferSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emmccore_buffer::R`](R) reader structure"]
impl crate::Readable for EmmccoreBufferSpec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_buffer::W`](W) writer structure"]
impl crate::Writable for EmmccoreBufferSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_BUFFER to value 0"]
impl crate::Resettable for EmmccoreBufferSpec {
    const RESET_VALUE: u32 = 0;
}

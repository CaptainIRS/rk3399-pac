#[doc = "Register `MEMORY_BASE_MEMORY_LIMIT` reader"]
pub type R = crate::R<MemoryBaseMemoryLimitSpec>;
#[doc = "Register `MEMORY_BASE_MEMORY_LIMIT` writer"]
pub type W = crate::W<MemoryBaseMemoryLimitSpec>;
#[doc = "Field `R1` reader - Reserved \\[R1\\]
Reserved"]
pub type R1R = crate::FieldReader;
#[doc = "Field `MBR` reader - Memory Base Register \\[MBR\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
pub type MbrR = crate::FieldReader<u16>;
#[doc = "Field `MBR` writer - Memory Base Register \\[MBR\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
pub type MbrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `R2` reader - Reserved \\[R2\\]
Reserved"]
pub type R2R = crate::FieldReader;
#[doc = "Field `MLR` reader - Memory Limit Register \\[MLR\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
pub type MlrR = crate::FieldReader<u16>;
#[doc = "Field `MLR` writer - Memory Limit Register \\[MLR\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
pub type MlrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:3 - Reserved \\[R1\\]
Reserved"]
    #[inline(always)]
    pub fn r1(&self) -> R1R {
        R1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Memory Base Register \\[MBR\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
    #[inline(always)]
    pub fn mbr(&self) -> MbrR {
        MbrR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - Reserved \\[R2\\]
Reserved"]
    #[inline(always)]
    pub fn r2(&self) -> R2R {
        R2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - Memory Limit Register \\[MLR\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
    #[inline(always)]
    pub fn mlr(&self) -> MlrR {
        MlrR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:15 - Memory Base Register \\[MBR\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
    #[inline(always)]
    #[must_use]
    pub fn mbr(&mut self) -> MbrW<MemoryBaseMemoryLimitSpec> {
        MbrW::new(self, 4)
    }
    #[doc = "Bits 20:31 - Memory Limit Register \\[MLR\\]
This field can be read and written from the local management APB bus, but its value is not used within the core."]
    #[inline(always)]
    #[must_use]
    pub fn mlr(&mut self) -> MlrW<MemoryBaseMemoryLimitSpec> {
        MlrW::new(self, 20)
    }
}
#[doc = "Memory Base, Memory Limit This field can be read and written from the local management APB bus, but its value is not used within the core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memory_base_memory_limit::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`memory_base_memory_limit::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemoryBaseMemoryLimitSpec;
impl crate::RegisterSpec for MemoryBaseMemoryLimitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memory_base_memory_limit::R`](R) reader structure"]
impl crate::Readable for MemoryBaseMemoryLimitSpec {}
#[doc = "`write(|w| ..)` method takes [`memory_base_memory_limit::W`](W) writer structure"]
impl crate::Writable for MemoryBaseMemoryLimitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMORY_BASE_MEMORY_LIMIT to value 0"]
impl crate::Resettable for MemoryBaseMemoryLimitSpec {
    const RESET_VALUE: u32 = 0;
}

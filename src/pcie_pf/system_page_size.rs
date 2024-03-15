#[doc = "Register `SYSTEM_PAGE_SIZE` reader"]
pub type R = crate::R<SystemPageSizeSpec>;
#[doc = "Register `SYSTEM_PAGE_SIZE` writer"]
pub type W = crate::W<SystemPageSizeSpec>;
#[doc = "Field `SPS` reader - System Page Size \\[SPS\\]
This field must be programmed by software to the current page size in use. The core implements only bits 15:0 of this register. This field can also be written from the local management bus."]
pub type SpsR = crate::FieldReader<u16>;
#[doc = "Field `SPS` writer - System Page Size \\[SPS\\]
This field must be programmed by software to the current page size in use. The core implements only bits 15:0 of this register. This field can also be written from the local management bus."]
pub type SpsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `R0` reader - Reserved \\[R0\\]
Reserved"]
pub type R0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - System Page Size \\[SPS\\]
This field must be programmed by software to the current page size in use. The core implements only bits 15:0 of this register. This field can also be written from the local management bus."]
    #[inline(always)]
    pub fn sps(&self) -> SpsR {
        SpsR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Reserved \\[R0\\]
Reserved"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - System Page Size \\[SPS\\]
This field must be programmed by software to the current page size in use. The core implements only bits 15:0 of this register. This field can also be written from the local management bus."]
    #[inline(always)]
    #[must_use]
    pub fn sps(&mut self) -> SpsW<SystemPageSizeSpec> {
        SpsW::new(self, 0)
    }
}
#[doc = "System Page Size Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`system_page_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`system_page_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SystemPageSizeSpec;
impl crate::RegisterSpec for SystemPageSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`system_page_size::R`](R) reader structure"]
impl crate::Readable for SystemPageSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`system_page_size::W`](W) writer structure"]
impl crate::Writable for SystemPageSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSTEM_PAGE_SIZE to value 0x01"]
impl crate::Resettable for SystemPageSizeSpec {
    const RESET_VALUE: u32 = 0x01;
}

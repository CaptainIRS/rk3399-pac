#[doc = "Register `ZAP_ONE_LINE` reader"]
pub type R = crate::R<ZapOneLineSpec>;
#[doc = "Register `ZAP_ONE_LINE` writer"]
pub type W = crate::W<ZapOneLineSpec>;
#[doc = "Field `MMU_ZAP_ONE_LINE` reader - address to be invalidated from the page table\n\ncache."]
pub type MmuZapOneLineR = crate::FieldReader<u32>;
#[doc = "Field `MMU_ZAP_ONE_LINE` writer - address to be invalidated from the page table\n\ncache."]
pub type MmuZapOneLineW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - address to be invalidated from the page table\n\ncache."]
    #[inline(always)]
    pub fn mmu_zap_one_line(&self) -> MmuZapOneLineR {
        MmuZapOneLineR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - address to be invalidated from the page table\n\ncache."]
    #[inline(always)]
    #[must_use]
    pub fn mmu_zap_one_line(&mut self) -> MmuZapOneLineW<ZapOneLineSpec> {
        MmuZapOneLineW::new(self, 0)
    }
}
#[doc = "MMU zap cache line register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`zap_one_line::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`zap_one_line::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ZapOneLineSpec;
impl crate::RegisterSpec for ZapOneLineSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`zap_one_line::R`](R) reader structure"]
impl crate::Readable for ZapOneLineSpec {}
#[doc = "`write(|w| ..)` method takes [`zap_one_line::W`](W) writer structure"]
impl crate::Writable for ZapOneLineSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ZAP_ONE_LINE to value 0"]
impl crate::Resettable for ZapOneLineSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `DMAC_DBGCMD` writer"]
pub type W = crate::W<DmacDbgcmdSpec>;
#[doc = "Field `DMAC_DBGCMD_BITS_1` writer - The debug encoding is as follows:\n\nb00 = execute the instruction that the DBGINST \\[1:0\\]
Registers\n\ncontain\n\nb01 = reserved\n\nb10 = reserved\n\nb11 = reserved"]
pub type DmacDbgcmdBits1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl W {
    #[doc = "Bits 0:1 - The debug encoding is as follows:\n\nb00 = execute the instruction that the DBGINST \\[1:0\\]
Registers\n\ncontain\n\nb01 = reserved\n\nb10 = reserved\n\nb11 = reserved"]
    #[inline(always)]
    #[must_use]
    pub fn dmac_dbgcmd_bits_1(&mut self) -> DmacDbgcmdBits1W<DmacDbgcmdSpec> {
        DmacDbgcmdBits1W::new(self, 0)
    }
}
#[doc = "Debug Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_dbgcmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacDbgcmdSpec;
impl crate::RegisterSpec for DmacDbgcmdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dmac_dbgcmd::W`](W) writer structure"]
impl crate::Writable for DmacDbgcmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAC_DBGCMD to value 0"]
impl crate::Resettable for DmacDbgcmdSpec {
    const RESET_VALUE: u32 = 0;
}

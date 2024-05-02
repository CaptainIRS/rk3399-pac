#[doc = "Register `DPCC_BPT_ADDR` reader"]
pub type R = crate::R<DpccBptAddrSpec>;
#[doc = "Register `DPCC_BPT_ADDR` writer"]
pub type W = crate::W<DpccBptAddrSpec>;
#[doc = "Field `bp_table_addr` reader - Table RAM start address for read or write operations.\n\nThe address counter is incremented at each read or write\n\naccess to the data register (auto-increment\n\nmechanism)."]
pub type BpTableAddrR = crate::FieldReader<u16>;
#[doc = "Field `bp_table_addr` writer - Table RAM start address for read or write operations.\n\nThe address counter is incremented at each read or write\n\naccess to the data register (auto-increment\n\nmechanism)."]
pub type BpTableAddrW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Table RAM start address for read or write operations.\n\nThe address counter is incremented at each read or write\n\naccess to the data register (auto-increment\n\nmechanism)."]
    #[inline(always)]
    pub fn bp_table_addr(&self) -> BpTableAddrR {
        BpTableAddrR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Table RAM start address for read or write operations.\n\nThe address counter is incremented at each read or write\n\naccess to the data register (auto-increment\n\nmechanism)."]
    #[inline(always)]
    #[must_use]
    pub fn bp_table_addr(&mut self) -> BpTableAddrW<DpccBptAddrSpec> {
        BpTableAddrW::new(self, 0)
    }
}
#[doc = "TABLE Start Address for table-based correction algorithm\n\nNote: MKOE tbc: Orignial register mode was rwh which is no longer supported with new \n\n\n\nversion of SIG \n\n\n\n-> rwhh \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpcc_bpt_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpcc_bpt_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpccBptAddrSpec;
impl crate::RegisterSpec for DpccBptAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpcc_bpt_addr::R`](R) reader structure"]
impl crate::Readable for DpccBptAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dpcc_bpt_addr::W`](W) writer structure"]
impl crate::Writable for DpccBptAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPCC_BPT_ADDR to value 0"]
impl crate::Resettable for DpccBptAddrSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `LSC_GR_TABLE_ADDR` reader"]
pub type R = crate::R<LscGrTableAddrSpec>;
#[doc = "Register `LSC_GR_TABLE_ADDR` writer"]
pub type W = crate::W<LscGrTableAddrSpec>;
#[doc = "Field `gr_ram_addr` reader - table address in RAM for samples of the G_R color\n\ncomponent. Will be automatically incremented by each\n\nread or write access to the table."]
pub type GrRamAddrR = crate::FieldReader<u16>;
#[doc = "Field `gr_ram_addr` writer - table address in RAM for samples of the G_R color\n\ncomponent. Will be automatically incremented by each\n\nread or write access to the table."]
pub type GrRamAddrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - table address in RAM for samples of the G_R color\n\ncomponent. Will be automatically incremented by each\n\nread or write access to the table."]
    #[inline(always)]
    pub fn gr_ram_addr(&self) -> GrRamAddrR {
        GrRamAddrR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - table address in RAM for samples of the G_R color\n\ncomponent. Will be automatically incremented by each\n\nread or write access to the table."]
    #[inline(always)]
    #[must_use]
    pub fn gr_ram_addr(&mut self) -> GrRamAddrW<LscGrTableAddrSpec> {
        GrRamAddrW::new(self, 0)
    }
}
#[doc = "Table RAM Address for green (red) component\n\nNote: MKOE tbc: Orignial register mode was rwh which is no longer supported with new \n\nversion of SIG-> rwhh Table set 0 access by SW at table address 0...153. Table set 1 access at \n\ntable address 154...307. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_gr_table_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_gr_table_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LscGrTableAddrSpec;
impl crate::RegisterSpec for LscGrTableAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsc_gr_table_addr::R`](R) reader structure"]
impl crate::Readable for LscGrTableAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`lsc_gr_table_addr::W`](W) writer structure"]
impl crate::Writable for LscGrTableAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LSC_GR_TABLE_ADDR to value 0"]
impl crate::Resettable for LscGrTableAddrSpec {
    const RESET_VALUE: u32 = 0;
}

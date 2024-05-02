#[doc = "Register `LSC_R_TABLE_ADDR` reader"]
pub type R = crate::R<LscRTableAddrSpec>;
#[doc = "Register `LSC_R_TABLE_ADDR` writer"]
pub type W = crate::W<LscRTableAddrSpec>;
#[doc = "Field `r_ram_addr` reader - table address in RAM for samples of the R color\n\ncomponent. Will be automatically incremented by each\n\nread or write access to the table.\n\nValid addresses are in the range 0 to 152."]
pub type RRamAddrR = crate::FieldReader<u16>;
#[doc = "Field `r_ram_addr` writer - table address in RAM for samples of the R color\n\ncomponent. Will be automatically incremented by each\n\nread or write access to the table.\n\nValid addresses are in the range 0 to 152."]
pub type RRamAddrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - table address in RAM for samples of the R color\n\ncomponent. Will be automatically incremented by each\n\nread or write access to the table.\n\nValid addresses are in the range 0 to 152."]
    #[inline(always)]
    pub fn r_ram_addr(&self) -> RRamAddrR {
        RRamAddrR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - table address in RAM for samples of the R color\n\ncomponent. Will be automatically incremented by each\n\nread or write access to the table.\n\nValid addresses are in the range 0 to 152."]
    #[inline(always)]
    #[must_use]
    pub fn r_ram_addr(&mut self) -> RRamAddrW<LscRTableAddrSpec> {
        RRamAddrW::new(self, 0)
    }
}
#[doc = "Table RAM Address for red component\n\nNote: The table values are written into an internal RAM. The RAM Address is generated per \n\nauto- increment. The tables values will be read back by a continuous read access to the \n\ncorresponding register. The read address is auto-incremented for each read access to that \n\nregister and is reset to a specific value by a write access to the ISP_LSC_TABLE_ADDR \n\nregister. \n\n\n\nTable set 0 access by SW at table address 0...152. Table set 1 access at table address \n\n153...305. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsc_r_table_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsc_r_table_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LscRTableAddrSpec;
impl crate::RegisterSpec for LscRTableAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsc_r_table_addr::R`](R) reader structure"]
impl crate::Readable for LscRTableAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`lsc_r_table_addr::W`](W) writer structure"]
impl crate::Writable for LscRTableAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LSC_R_TABLE_ADDR to value 0"]
impl crate::Resettable for LscRTableAddrSpec {
    const RESET_VALUE: u32 = 0;
}

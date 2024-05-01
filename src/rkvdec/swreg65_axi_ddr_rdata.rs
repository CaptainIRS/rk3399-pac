#[doc = "Register `SWREG65_AXI_DDR_RDATA` reader"]
pub type R = crate::R<Swreg65AxiDdrRdataSpec>;
#[doc = "Register `SWREG65_AXI_DDR_RDATA` writer"]
pub type W = crate::W<Swreg65AxiDdrRdataSpec>;
#[doc = "Field `SW_AXI_DDR_RDATA` reader - axi ddr rdata num\n\naxi ddr rdata num, the unit is byte"]
pub type SwAxiDdrRdataR = crate::FieldReader<u32>;
#[doc = "Field `SW_AXI_DDR_RDATA` writer - axi ddr rdata num\n\naxi ddr rdata num, the unit is byte"]
pub type SwAxiDdrRdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - axi ddr rdata num\n\naxi ddr rdata num, the unit is byte"]
    #[inline(always)]
    pub fn sw_axi_ddr_rdata(&self) -> SwAxiDdrRdataR {
        SwAxiDdrRdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - axi ddr rdata num\n\naxi ddr rdata num, the unit is byte"]
    #[inline(always)]
    #[must_use]
    pub fn sw_axi_ddr_rdata(&mut self) -> SwAxiDdrRdataW<Swreg65AxiDdrRdataSpec> {
        SwAxiDdrRdataW::new(self, 0)
    }
}
#[doc = "axi ddr read data num\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg65_axi_ddr_rdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg65_axi_ddr_rdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg65AxiDdrRdataSpec;
impl crate::RegisterSpec for Swreg65AxiDdrRdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg65_axi_ddr_rdata::R`](R) reader structure"]
impl crate::Readable for Swreg65AxiDdrRdataSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg65_axi_ddr_rdata::W`](W) writer structure"]
impl crate::Writable for Swreg65AxiDdrRdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG65_AXI_DDR_RDATA to value 0"]
impl crate::Resettable for Swreg65AxiDdrRdataSpec {
    const RESET_VALUE: u32 = 0;
}

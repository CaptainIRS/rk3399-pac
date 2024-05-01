#[doc = "Register `SWREG66_AXI_DDR_WDATA` reader"]
pub type R = crate::R<Swreg66AxiDdrWdataSpec>;
#[doc = "Register `SWREG66_AXI_DDR_WDATA` writer"]
pub type W = crate::W<Swreg66AxiDdrWdataSpec>;
#[doc = "Field `SW_AXI_DDR_WDATA` reader - hevc write data byte num\n\nhevc write data byte num"]
pub type SwAxiDdrWdataR = crate::FieldReader<u32>;
#[doc = "Field `SW_AXI_DDR_WDATA` writer - hevc write data byte num\n\nhevc write data byte num"]
pub type SwAxiDdrWdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - hevc write data byte num\n\nhevc write data byte num"]
    #[inline(always)]
    pub fn sw_axi_ddr_wdata(&self) -> SwAxiDdrWdataR {
        SwAxiDdrWdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - hevc write data byte num\n\nhevc write data byte num"]
    #[inline(always)]
    #[must_use]
    pub fn sw_axi_ddr_wdata(&mut self) -> SwAxiDdrWdataW<Swreg66AxiDdrWdataSpec> {
        SwAxiDdrWdataW::new(self, 0)
    }
}
#[doc = "axi ddr write data number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg66_axi_ddr_wdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg66_axi_ddr_wdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg66AxiDdrWdataSpec;
impl crate::RegisterSpec for Swreg66AxiDdrWdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg66_axi_ddr_wdata::R`](R) reader structure"]
impl crate::Readable for Swreg66AxiDdrWdataSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg66_axi_ddr_wdata::W`](W) writer structure"]
impl crate::Writable for Swreg66AxiDdrWdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG66_AXI_DDR_WDATA to value 0"]
impl crate::Resettable for Swreg66AxiDdrWdataSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `DDRC0_CON1` reader"]
pub type R = crate::R<Ddrc0Con1Spec>;
#[doc = "Register `DDRC0_CON1` writer"]
pub type W = crate::W<Ddrc0Con1Spec>;
#[doc = "Field `CLK_DDRC0_EN_STDBY` reader - bit control of clk_ddrc0_en_stdby"]
pub type ClkDdrc0EnStdbyR = crate::BitReader;
#[doc = "Field `CLK_DDRC0_EN_STDBY` writer - bit control of clk_ddrc0_en_stdby"]
pub type ClkDdrc0EnStdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DDRPHY_CTRL0_EN_STDBY` reader - bit control of clk_ddrc0_en_stdby"]
pub type ClkDdrphyCtrl0EnStdbyR = crate::BitReader;
#[doc = "Field `CLK_DDRPHY_CTRL0_EN_STDBY` writer - bit control of clk_ddrc0_en_stdby"]
pub type ClkDdrphyCtrl0EnStdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DDRPHY0_EN_STDBY` reader - bit control of clk_ddrphy0_en_stdby"]
pub type ClkDdrphy0EnStdbyR = crate::BitReader;
#[doc = "Field `CLK_DDRPHY0_EN_STDBY` writer - bit control of clk_ddrphy0_en_stdby"]
pub type ClkDdrphy0EnStdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DDR0_MSCH_EN_STDBY` reader - bit control of clk_ddr0_msch_en_stdby"]
pub type ClkDdr0MschEnStdbyR = crate::BitReader;
#[doc = "Field `CLK_DDR0_MSCH_EN_STDBY` writer - bit control of clk_ddr0_msch_en_stdby"]
pub type ClkDdr0MschEnStdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DENALI0_COMMAND_PRIORITY` reader - bit control of denali0_command_priority"]
pub type Denali0CommandPriorityR = crate::FieldReader;
#[doc = "Field `DENALI0_COMMAND_PRIORITY` writer - bit control of denali0_command_priority"]
pub type Denali0CommandPriorityW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 4 - bit control of clk_ddrc0_en_stdby"]
    #[inline(always)]
    pub fn clk_ddrc0_en_stdby(&self) -> ClkDdrc0EnStdbyR {
        ClkDdrc0EnStdbyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - bit control of clk_ddrc0_en_stdby"]
    #[inline(always)]
    pub fn clk_ddrphy_ctrl0_en_stdby(&self) -> ClkDdrphyCtrl0EnStdbyR {
        ClkDdrphyCtrl0EnStdbyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - bit control of clk_ddrphy0_en_stdby"]
    #[inline(always)]
    pub fn clk_ddrphy0_en_stdby(&self) -> ClkDdrphy0EnStdbyR {
        ClkDdrphy0EnStdbyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - bit control of clk_ddr0_msch_en_stdby"]
    #[inline(always)]
    pub fn clk_ddr0_msch_en_stdby(&self) -> ClkDdr0MschEnStdbyR {
        ClkDdr0MschEnStdbyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - bit control of denali0_command_priority"]
    #[inline(always)]
    pub fn denali0_command_priority(&self) -> Denali0CommandPriorityR {
        Denali0CommandPriorityR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 4 - bit control of clk_ddrc0_en_stdby"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ddrc0_en_stdby(&mut self) -> ClkDdrc0EnStdbyW<Ddrc0Con1Spec> {
        ClkDdrc0EnStdbyW::new(self, 4)
    }
    #[doc = "Bit 5 - bit control of clk_ddrc0_en_stdby"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ddrphy_ctrl0_en_stdby(&mut self) -> ClkDdrphyCtrl0EnStdbyW<Ddrc0Con1Spec> {
        ClkDdrphyCtrl0EnStdbyW::new(self, 5)
    }
    #[doc = "Bit 6 - bit control of clk_ddrphy0_en_stdby"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ddrphy0_en_stdby(&mut self) -> ClkDdrphy0EnStdbyW<Ddrc0Con1Spec> {
        ClkDdrphy0EnStdbyW::new(self, 6)
    }
    #[doc = "Bit 7 - bit control of clk_ddr0_msch_en_stdby"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ddr0_msch_en_stdby(&mut self) -> ClkDdr0MschEnStdbyW<Ddrc0Con1Spec> {
        ClkDdr0MschEnStdbyW::new(self, 7)
    }
    #[doc = "Bits 8:9 - bit control of denali0_command_priority"]
    #[inline(always)]
    #[must_use]
    pub fn denali0_command_priority(&mut self) -> Denali0CommandPriorityW<Ddrc0Con1Spec> {
        Denali0CommandPriorityW::new(self, 8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Ddrc0Con1Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "ddrc0 control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrc0_con1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrc0_con1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ddrc0Con1Spec;
impl crate::RegisterSpec for Ddrc0Con1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrc0_con1::R`](R) reader structure"]
impl crate::Readable for Ddrc0Con1Spec {}
#[doc = "`write(|w| ..)` method takes [`ddrc0_con1::W`](W) writer structure"]
impl crate::Writable for Ddrc0Con1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRC0_CON1 to value 0"]
impl crate::Resettable for Ddrc0Con1Spec {
    const RESET_VALUE: u32 = 0;
}

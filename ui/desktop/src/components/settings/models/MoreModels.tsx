import React, { useState } from 'react';
import { Button } from "../../ui/button"
import { RecentModels } from "./RecentModels"
import { ProviderButtons } from "./ProviderButtons"
import { AddModelDialog } from "./AddModelDialog"
import BackButton from "../../ui/BackButton";
import { SearchBar} from "./Search";
import { useModel} from "./ModelContext";


// TODO: handle darkmode
export default function MoreModelsPage() {
    const { currentModel } = useModel(); // Access global state

    return (
        <div className="flex min-h-screen bg-background text-foreground overflow-y-auto overflow-x-hidden">
            {/* Left-hand side exit button */}
            <aside className="w-48 border-r border-gray-100 dark:border-gray-700 px-2 pt-6">
                <div className="sticky top-8">
                    <BackButton />
                </div>
            </aside>

            <div className="container max-w-6xl mx-auto p-6">
                {/* First row: Title and buttons */}
                <div className="flex justify-between items-center mb-4">
                    <h1 className="text-2xl font-semibold">More Models</h1>

                    <div className="flex items-center space-x-4">
                        <Button
                            variant="outline"
                            onClick={() => console.log("Navigate to Configure Providers")}
                        >
                            Configure Providers
                        </Button>
                        <AddModelDialog />
                    </div>
                </div>

                {/* Second row: Current model */}
                {currentModel && (
                    <div className="mb-8">
                        <p className="text-sm text-muted-foreground">
                            Current model: <span className="font-medium">{currentModel.name}</span> ({currentModel.provider})
                        </p>
                    </div>
                )}

                {/* Main content area */}
                <div className="space-y-8">
                    {/* Search section */}
                    <SearchBar />

                    {/* Provider buttons */}
                    <div className="space-y-4">
                        <h2 className="text-lg font-medium">Browse by Provider</h2>
                        <ProviderButtons />
                    </div>

                    {/* Recent models */}
                    <div className="space-y-4">
                        <div className="flex items-center justify-between">
                            <h2 className="text-lg font-medium">Recently Used Models</h2>
                            <Button variant="ghost" className="text-blue-500 hover:text-blue-600">
                                View all
                            </Button>
                        </div>
                        <RecentModels />
                    </div>
                </div>
            </div>
        </div>
    );
}

